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

pub type Value = u128;

pub use pallet::*;

/// Single transaction to be dispatched
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct Transaction {
    /// UTXOs to be used as inputs for current transaction
    pub inputs: Vec<TransactionInput>,

    /// UTXOs to be created as a result of current transaction dispatch
    pub outputs: Vec<TransactionOutput>,
}

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
pub type GenesisUtxoType = (Value, H256);

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A source to determine the block author
        type BlockAuthor: BlockAuthor;

        /// A source to determine the issuance portion of the block reward
        type Issuance: Issuance<BlockNumberFor<Self>, Value>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Total reward value to be redistributed among authorities.
    /// It is accumulated from transactions during block execution
    /// and then dispersed to validators on block finalization.
    #[pallet::storage]
    #[pallet::getter(fn total_reward)]
    pub type TotalReward<T: Config> = StorageValue<_, Value, ValueQuery>;

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

    /// Keep track of latest UTXO hash of account
    /// Mapping from `sr25519::Pubkey` to `BlakeTwo256::hash_of(transaction, index)`
    /// Just for testing 🫤
    /// Because 1 account may have multiple UTXOs
    #[pallet::storage]
    #[pallet::getter(fn utxo_of)]
    pub type UtxoOf<T: Config> =
        StorageMap<Hasher = Identity, Key = Public, Value = H256, QueryKind = OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _ph_data: Option<PhantomData<T>>,
        pub genesis_utxos: Vec<GenesisUtxoType>,
    }

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

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                _ph_data: Default::default(),
                genesis_utxos: Vec::<GenesisUtxoType>::new(),
            }
        }
    }

    /// Pallets use events to inform users when important changes are made.
    /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error
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

    /// Errors inform users that something went wrong.
    /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn spend(_origin: OriginFor<T>, transaction: Transaction) -> DispatchResult {
            let transaction_validity = Self::validate_transaction(&transaction)?;
            ensure!(
                transaction_validity.requires.is_empty(),
                Error::<T>::MissingInput
            );

            Self::update_storage(&transaction, transaction_validity.priority as Value)?;

            Self::deposit_event(Event::<T>::TransactionSuccess(transaction));

            Ok(())
        }

        pub fn faucet(_origin: OriginFor<T>, to: Public, value: Value) -> DispatchResult {
            let _ = Self::deposit_creating(&to, value);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Update storage to reflect changes made by transaction
        /// Where each utxo key is a hash of the entire transaction and its order in the TransactionOutputs vector
        fn update_storage(transaction: &Transaction, reward: Value) -> DispatchResult {
            // Calculate new reward total
            let new_total = TotalReward::<T>::get()
                .checked_add(reward)
                .ok_or(Error::<T>::RewardOverflow)?;
            TotalReward::<T>::put(new_total);

            // Removing spent UTXOs
            for input in &transaction.inputs {
                UtxoStore::<T>::remove(input.outpoint);
            }

            let mut index: u64 = 0;
            log::info!("transaction outputs len: {:?}", transaction.outputs.len());
            for output in &transaction.outputs {
                let hash = BlakeTwo256::hash_of(&(&transaction.encode(), index));
                log::info!("hash: {:?}", hash);
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

        fn deposit_creating(to: &Public, value: Value) -> DispatchResult {
            let utxo = TransactionOutput {
                value,
                pubkey: H256::from_slice(&to[..]),
            };

            let hash = BlakeTwo256::hash_of(&(&utxo, 0));
            Self::store_utxo(&utxo, hash);

            Self::deposit_event(Event::<T>::Faucet(value, hash));

            Ok(())
        }

        fn store_utxo(utxo: &TransactionOutput, hash: H256) {
            UtxoStore::<T>::insert(hash, utxo);
            // Convert H256 back to sr25519::Public
            let pubkey = Public::from_h256(utxo.pubkey);
            UtxoOf::<T>::insert(pubkey, hash);
        }

        // Strips a transaction of its Signature fields by replacing value with ZERO-initialized fixed hash.
        fn get_simple_transaction(transaction: &Transaction) -> Vec<u8> {
            //&'a [u8] {
            let mut trx = transaction.clone();
            for input in trx.inputs.iter_mut() {
                input.sigscript = H512::zero();
            }

            trx.encode()
        }

        // /// Helper fn for Transaction Pool
        // /// Checks for race condition, if a certain trx is missing input_utxos in UtxoStore
        // /// If None missing inputs: no race condition, gtg
        // /// if Some(missing inputs): there are missing variables
        // fn get_missing_utxos(transaction: &Transaction) -> Vec<&H256> {
        //     let mut missing_utxos = Vec::new();
        //     for input in transaction.inputs.iter() {
        //         if UtxoStore::<T>::get(&input.outpoint).is_none() {
        //             missing_utxos.push(&input.outpoint);
        //         }
        //     }
        //     missing_utxos
        // }

        /// Check transaction for validity, errors, & race conditions
        /// Called by both transaction pool and runtime execution
        ///
        /// Ensures that:
        /// - inputs and outputs are not empty
        /// - all inputs match to existing, unspent and unlocked outputs
        /// - each input is used exactly once
        /// - each output is defined exactly once and has nonzero value
        /// - total output value must not exceed total input value
        /// - new outputs do not collide with existing ones
        /// - sum of input and output values does not overflow
        /// - provided signatures are valid
        /// - transaction outputs cannot be modified by malicious nodes
        pub fn validate_transaction(
            transaction: &Transaction,
        ) -> Result<ValidTransaction, &'static str> {
            // Check basic requirements
            ensure!(!transaction.inputs.is_empty(), Error::<T>::EmptyInput);
            ensure!(!transaction.outputs.is_empty(), Error::<T>::EmptyOutput);

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

            // Check that inputs are valid
            for input in transaction.inputs.iter() {
                if let Some(input_utxo) = UtxoStore::<T>::get(&input.outpoint) {
                    log::info!("encoded tx: {:?}", simple_transaction);
                    let is_valid_sig = sp_io::crypto::sr25519_verify(
                        &Signature::from_raw(*input.sigscript.as_fixed_bytes()),
                        &simple_transaction,
                        &Public::from_h256(input_utxo.pubkey),
                    );
                    log::info!("is_valid_sig: {:?}", is_valid_sig);
                    ensure!(is_valid_sig, Error::<T>::InvalidSignature);
                    total_input = total_input
                        .checked_add(input_utxo.value)
                        .ok_or(Error::<T>::InputOverflow)?;
                } else {
                    missing_utxos.push(input.outpoint.clone().as_fixed_bytes().to_vec());
                }
            }

            // Check that outputs are valid
            for output in transaction.outputs.iter() {
                ensure!(output.value > 0, Error::<T>::ZeroAmount);
                let hash = BlakeTwo256::hash_of(&(&transaction.encode(), output_index));
                output_index = output_index
                    .checked_add(1)
                    .ok_or(Error::<T>::MaximumTransactionDepth)?;
                ensure!(
                    !UtxoStore::<T>::contains_key(hash),
                    Error::<T>::DuplicatedOutput
                );
                total_output = total_output
                    .checked_add(output.value)
                    .ok_or(Error::<T>::OutputOverflow)?;
                new_utxos.push(hash.as_fixed_bytes().to_vec());
            }

            // If no race condition, check the math
            if missing_utxos.is_empty() {
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
