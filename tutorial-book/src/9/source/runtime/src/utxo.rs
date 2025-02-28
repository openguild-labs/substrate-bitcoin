// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{
    sr25519::{Public, Signature},
    ByteArray, H256, H512,
};
use sp_runtime::traits::{BlakeTwo256, Hash, SaturatedConversion};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use super::{block_author::BlockAuthor, issuance::Issuance};

pub use pallet::*;

/// [2-data-structure]
pub type Value = u128;

/// [2-data-structure]
/// Single transaction to be dispatched
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct Transaction {
    /// UTXOs to be used as inputs for current transaction
    pub inputs: Vec<TransactionInput>,
    /// UTXOs to be created as a result of current transaction dispatch
    pub outputs: Vec<TransactionOutput>,
}

/// [2-data-structure]
/// Single transaction input that refers to one UTXO
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct TransactionInput {
    /// Reference to an UTXO to be spent
    pub outpoint: H256,

    /// Proof that transaction owner is authorized to spend referred UTXO &
    /// that the entire transaction is untampered
    pub sigscript: H512,
}

/// [2-data-structure]
/// Single transaction output to create upon transaction dispatch
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct TransactionOutput {
    /// Value associated with this output
    pub value: Value,

    /// Public key associated with this output. In order to spend this output
    /// owner must provide a proof by hashing the whole `Transaction` and
    /// signing it with a corresponding private key.
    pub pubkey: H256,
}


/// [6-genesis-builder]
/// Because code is built on `no-std` feature.
/// And we got error:
/// ```
/// ...
/// the trait `Serialize` is not implemented for `TransactionOutput`
/// the trait `Deserialize<'_>` is not implemented for `TransactionOutput`
/// ...
/// ```
///
/// Hence, we need to simplify data type to make it work in both `std` and `no-std` feature.
/// Genesis Utxo Type
pub type GenesisUtxoType = (Value, H256);

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    /// [2-data-structure]
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// Read more: https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A source to determine the block author
        type BlockAuthor: BlockAuthor;

        /// A source to determine the issuance portion of the block reward
        type Issuance: Issuance<BlockNumberFor<Self>, Value>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);


    /// [2-data-structure]
    /// Total reward value to be redistributed among authorities.
    /// It is accumulated from transactions during block execution
    /// and then dispersed to validators on block finalization.
    #[pallet::storage]
    #[pallet::getter(fn total_reward)]
    pub type TotalReward<T: Config> = StorageValue<_, Value, ValueQuery>;

    /// [2-data-structure]
    /// All valid unspent transaction outputs are stored in this map.
    /// Initial set of UTXO is populated from the list stored in genesis.
    /// We use the identity hasher here because the cryptographic hashing is
    /// done explicitly.
    /// Mapping from `BlakeTwo256::hash_of(transaction, index)` to `TransactionOutput`
    #[pallet::storage]
    #[pallet::getter(fn utxo_store)]
    pub type UtxoStore<T: Config> = StorageMap<
        Hasher = Identity,
        Key = H256,
        Value = TransactionOutput,
        QueryKind = OptionQuery,
    >;

    /// [6-genesis-builder]
    /// Keep track of latest UTXO hash of account
    /// Mapping from `sr25519::Pubkey` to `BlakeTwo256::hash_of(transaction, index)`
    /// Just for testing 🫤
    /// Because 1 account may have multiple UTXOs
    #[pallet::storage]
    #[pallet::getter(fn utxo_of)]
    pub type UtxoOf<T: Config> =
        StorageMap<Hasher = Identity, Key = Public, Value = H256, QueryKind = OptionQuery>;

    /// [6-genesis-builder]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _ph_data: Option<PhantomData<T>>,
        pub genesis_utxos: Vec<GenesisUtxoType>,
    }

    /// [6-genesis-builder]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                _ph_data: Default::default(),
                genesis_utxos: Vec::<GenesisUtxoType>::new(),
            }
        }
    }

    /// [6-genesis-builder]
    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for utxo in self.genesis_utxos.iter() {
                let utxo = TransactionOutput { value: utxo.0, pubkey: utxo.1 };
                let hash = BlakeTwo256::hash_of(&utxo);
                Pallet::<T>::store_utxo(&utxo, hash);
            }
        }
    }

    /// [2-data-structure]
    /// Pallets use events to inform users when important changes are made.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Dispatch transaction successful
        TransactionSuccess(Transaction),
        /// UTXO out processed
        TransactionOutputProcessed(H256),
        /// Reward distributed to `BlockAuthor`
        RewardDistributed(Value, H256),
        /// Faucet to `To`
        Faucet(Value, H256),
        /// No one get reward
        RewardWasted,
    }


    /// [2-data-structure]
    /// Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Missing `Transaction` Input
        MissingInput,
        /// Reward overflow
        RewardOverflow,
        /// Maximum transaction depth
        MaximumTransactionDepth,
        /// Empty input
        EmptyInput,
        /// Empty output
        EmptyOutput,
        /// Each input must only be used once
        DuplicatedInput,
        /// Each output must be defined only once
        DuplicatedOutput,
        /// Input value is overflow
        InputOverflow,
        /// Output value is overflow
        OutputOverflow,
        /// Output spent must lte than Input spent
        OutputOverInput,
        /// Zero amount spent
        ZeroAmount,
        /// Invalid signature
        InvalidSignature,
    }

    /// [4-dispersed-reward]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            match T::BlockAuthor::block_author() {
                // Block author did not provide key to claim reward
                None => Self::deposit_event(Event::RewardWasted),
                // Block author did provide key, so issue thir reward
                Some(author) => Self::disperse_reward(&author),
            }
        }
    }

    /// [2-data-structure]
    /// Define extrinsics / dispatchable function
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn spend(_origin: OriginFor<T>, transaction: Transaction) -> DispatchResult {
            // [3-spend-utxo]
            // validate transaction
            let transaction_validity = Self::validate_transaction(&transaction)?;
            ensure!(
                transaction_validity.requires.is_empty(),
                Error::<T>::MissingInput
            );

            // implement logic
            Self::do_spend(&transaction, transaction_validity.priority as Value)?;
            // emit event
            Self::deposit_event(Event::<T>::TransactionSuccess(transaction));

            Ok(())
        }
    }

    /// [2-data-structure]
    /// Define intrinsics
    impl<T: Config> Pallet<T> {
        /// Implement spend logic, update storage to reflect changes made by transaction
        /// Where each UTXO key is a hash of the entire transaction and its order in the `TransactionOutputs` vector
        fn do_spend(transaction: &Transaction, reward: Value) -> DispatchResult {
            // [3-spend-utxo]
            // Calculate new reward total. The rest of `total_input - total_output` will be used for block reward.
            let new_total = TotalReward::<T>::get()
                .checked_add(reward)
                .ok_or(Error::<T>::RewardOverflow)?;
            TotalReward::<T>::put(new_total);

            // Removing spent UTXOs
            for input in &transaction.inputs {
                UtxoStore::<T>::remove(input.outpoint);
            }

            let mut index: u64 = 0;
            for output in &transaction.outputs {
                let hash = BlakeTwo256::hash_of(&(&transaction.encode(), index));
                // validated before, this is safe
                index = index
                    .checked_add(1)
                    .ok_or(Error::<T>::MaximumTransactionDepth)
                    .unwrap();
                Self::store_utxo(output, hash);
                Self::deposit_event(Event::TransactionOutputProcessed(hash));
            }

            Ok(())
        }

        /// Redistribute combined reward value to block Author
        fn disperse_reward(author: &Public) {
            // [4-dispersed-reward]

            // take the rest of reward
            // plus issuance reward of current block number
            let reward = TotalReward::<T>::take()
            + T::Issuance::issuance(frame_system::Pallet::<T>::block_number());

            let utxo = TransactionOutput {
                value: reward,
                pubkey: H256::from_slice(author.as_slice()),
            };

            let hash = BlakeTwo256::hash_of(&(
                &utxo,
                frame_system::Pallet::<T>::block_number().saturated_into::<u64>(),
            ));

            Self::store_utxo(&utxo, hash);
            Self::deposit_event(Event::RewardDistributed(reward, hash));  
        }

        /// Mutate storage, insert / update new UTXOs
        fn store_utxo(utxo: &TransactionOutput, hash: H256) {
            // [3-spend-utxo]
            UtxoStore::<T>::insert(hash, utxo);
            
            // [6-genesis-builder]
            // Convert H256 back to sr25519::Public
            let pubkey = Public::from_h256(utxo.pubkey);
            UtxoOf::<T>::insert(pubkey, hash);
        }

        /// Strips a transaction of its Signature fields by replacing value with ZERO-initialized fixed hash.
        fn get_simple_transaction(transaction: &Transaction) -> Vec<u8> {
            // [3-spend-utxo]
            let mut trx = transaction.clone();
            for input in trx.inputs.iter_mut() {
                input.sigscript = H512::zero();
            }

            trx.encode()
        }

        /// Check transaction for validity, errors, & race conditions
        /// Called by both transaction pool and runtime execution
        pub fn validate_transaction(
            transaction: &Transaction,
        ) -> Result<ValidTransaction, &'static str> {
            // [3-spend-utxo]
            // Check inputs and outputs are not empty
            ensure!(!transaction.inputs.is_empty(), Error::<T>::EmptyInput);
            ensure!(!transaction.outputs.is_empty(), Error::<T>::EmptyOutput);

            // Check each input is used exactly once
            {
                let input_set: BTreeMap<_, ()> =
                    transaction.inputs.iter().map(|input| (input, ())).collect();
                ensure!(
                    input_set.len() == transaction.inputs.len(),
                    Error::<T>::DuplicatedInput
                );
            }
            {
                let output_set: BTreeMap<_, ()> = transaction
                    .outputs
                    .iter()
                    .map(|output| (output, ()))
                    .collect();
                ensure!(
                    output_set.len() == transaction.outputs.len(),
                    Error::<T>::DuplicatedOutput
                );
            }

            let mut total_input: Value = 0;
            let mut total_output: Value = 0;
            let mut output_index: u64 = 0;
            let simple_transaction = Self::get_simple_transaction(transaction);

            // Variables sent to transaction pool
            let mut missing_utxos = Vec::new();
            let mut new_utxos = Vec::new();
            let mut reward = 0;

            for input in transaction.inputs.iter() {
                // Check all inputs match to existing, unspent and unlocked outputs
                if let Some(input_utxo) = UtxoStore::<T>::get(&input.outpoint) {
                    // Check provided signatures are valid
                    let is_valid_sig = sp_io::crypto::sr25519_verify(
                        &Signature::from_raw(*input.sigscript.as_fixed_bytes()),
                        &simple_transaction,
                        &Public::from_h256(input_utxo.pubkey),
                    );
                    ensure!(is_valid_sig, Error::<T>::InvalidSignature);
                    // Check sum of input values does not overflow
                    total_input = total_input
                        .checked_add(input_utxo.value)
                        .ok_or(Error::<T>::InputOverflow)?;
                } else {
                    missing_utxos.push(input.outpoint.clone().as_fixed_bytes().to_vec());
                }
            }

            // Check each output is defined exactly once and has nonzero value
            for output in transaction.outputs.iter() {
                ensure!(output.value > 0, Error::<T>::ZeroAmount);
                let hash = BlakeTwo256::hash_of(&(&transaction.encode(), output_index));
                output_index = output_index
                    .checked_add(1)
                    .ok_or(Error::<T>::MaximumTransactionDepth)?;
                // Check new outputs do not collide with existing ones
                ensure!(
                    !UtxoStore::<T>::contains_key(hash),
                    Error::<T>::DuplicatedOutput
                );
                // Check sum of output values does not overflow
                total_output = total_output
                    .checked_add(output.value)
                    .ok_or(Error::<T>::OutputOverflow)?;
                new_utxos.push(hash.as_fixed_bytes().to_vec());
            }

            // If no race condition, check the math
            if missing_utxos.is_empty() {
                // Check total output value must not exceed total input value
                ensure!(total_input >= total_output, Error::<T>::OutputOverInput);
                reward = total_input
                    .checked_sub(total_output)
                    .ok_or(Error::<T>::RewardOverflow)?;
            }

            // Returns transaction details
            Ok(ValidTransaction {
                requires: missing_utxos,
                provides: new_utxos,
                priority: reward as u64,
                longevity: TransactionLongevity::max_value(),
                propagate: true,
            })
        }
    }
}