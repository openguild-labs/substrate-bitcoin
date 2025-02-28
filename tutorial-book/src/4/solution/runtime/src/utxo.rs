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

///  [2-data-structure]
pub type Value = u128;

///  [2-data-structure]
/// Single transaction to be dispatched
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct Transaction {
    /// UTXOs to be used as inputs for current transaction
    pub inputs: Vec<TransactionInput>,
    /// UTXOs to be created as a result of current transaction dispatch
    pub outputs: Vec<TransactionOutput>,
}

///  [2-data-structure]
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

///  [2-data-structure]
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

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    ///  [2-data-structure]
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// Read more: https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A source to determine the block author
        /// Read more: `runtime/src/block_author.rs`
        /// Pallet loosely coupling
        /// https://polkadot-blockchain-academy.github.io/pba-book/frame/coupling/page.html#loosely-coupled-pallets
        type BlockAuthor: BlockAuthor;

        /// A source to determine the issuance portion of the block reward
        type Issuance: Issuance<BlockNumberFor<Self>, Value>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);


    ///  [2-data-structure]
    /// Total reward value to be redistributed among authorities.
    /// It is accumulated from transactions during block execution
    /// and then dispersed to validators on block finalization.
    #[pallet::storage]
    #[pallet::getter(fn total_reward)]
    pub type TotalReward<T: Config> = StorageValue<_, Value, ValueQuery>;

    ///  [2-data-structure]
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


    ///  [2-data-structure]
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


    ///  [2-data-structure]
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


    ///  [2-data-structure]
    /// Define extrinsics / dispatchable function
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn spend(_origin: OriginFor<T>, transaction: Transaction) -> DispatchResult {
            todo!();
        }

    }

    ///  [2-data-structure]
    /// Define intrinsics
    impl<T: Config> Pallet<T> {
        /// Implement spend logic, update storage to reflect changes made by transaction
        /// Where each UTXO key is a hash of the entire transaction and its order in the `TransactionOutputs` vector
        fn do_spend(transaction: &Transaction, reward: Value) -> DispatchResult {
            todo!();
        }

        /// Redistribute combined reward value to block Author
        fn disperse_reward(author: &Public) {
            todo!();
        }

        /// Mutate storage, insert / update new UTXOs
        fn store_utxo(utxo: &TransactionOutput, hash: H256) {
            todo!();
        }

        /// Strips a transaction of its Signature fields by replacing value with ZERO-initialized fixed hash.
        fn get_simple_transaction(transaction: &Transaction) -> Vec<u8> {
            todo!();
        }

        /// Check transaction for validity, errors, & race conditions
        /// Called by both transaction pool and runtime execution
        pub fn validate_transaction(
            transaction: &Transaction,
        ) -> Result<ValidTransaction, &'static str> {
            todo!();
        }
    }
}