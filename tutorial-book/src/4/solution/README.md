# Creating UTXO Pallet

> [!Note]
> Resolve all "/// TODO" in `runtime/src/utxo.rs` to complete this step.

### Reading Materials

I would recommend you to read these materials below first before looking at the code implmentation of the data structures. These materials below cover very well the concepts of FRAME storage in Substrate development.

- [Substrate Tutorial - Use macros in a custom pallet](https://docs.substrate.io/tutorials/build-application-logic/use-macros-in-a-custom-pallet/)
- [OpenGuild Blog - Code breakdown pallet template (Vietnamese)](https://openguild.wtf/blog/polkadot/code-breakdown-pallet-template)
- [Polkadot Blockchain Academy - FRAME Storage lecture](https://polkadot-blockchain-academy.github.io/pba-book/frame/storage/page.html)
- [Substrate Docs - Runtime storage structure](https://docs.substrate.io/build/runtime-storage/)
- [Polkadot Blockchain Academy - Event and Error](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error)
- [Polkadot Blockchain Academy - Pallet coupling](https://polkadot-blockchain-academy.github.io/pba-book/frame/coupling/page.html)

### Data structures to work with Storage API

The FRAME Storage module simplifies access to these layered storage abstractions. You can use the FRAME storage data structures to read or write any value that can be encoded by the SCALE codec. The storage module provides the following types of storage structures:


- `StorageValue`: to store any single value, such as a u64.
    - [Type](https://paritytech.github.io/polkadot-sdk/master/frame_support/storage/types/struct.StorageValue.html)
    - [Trait](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageValue.html)
- `StorageMap`: to store a single key to value mapping, such as a specific account key to a specific balance value.
    - [Type](https://paritytech.github.io/polkadot-sdk/master/frame_support/storage/types/struct.StorageMap.html) 
    - [Trait](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageMap.html)
- `StorageDoubleMap`: to store values in a storage map with two keys as an optimization to efficiently remove all entries that have a common first key.
    - [Type](https://paritytech.github.io/polkadot-sdk/master/frame_support/storage/types/struct.StorageDoubleMap.html) 
    - [Trait](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageDoubleMap.html)
- `StorageNMap`: to store values in a map with any arbitrary number of keys.
    - [Type](https://paritytech.github.io/polkadot-sdk/master/frame_support/storage/types/struct.StorageNMap.html) 
    - [Trait](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageNMap.html)


### Struct data for UTXO

Simple type for storing balance of UTXO

```rust
pub type Value = u128;
```

---

Struct for representing a transaction in a UTXO-based model. The struct includes macros for serialization, deserialization, and various traits that enable efficient use in a blockchain context. Let’s break down the purpose of each macro:

```rust
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct Transaction {
    /// UTXOs to be used as inputs for current transaction
    pub inputs: Vec<TransactionInput>,
    /// UTXOs to be created as a result of current transaction dispatch
    pub outputs: Vec<TransactionOutput>,
}


#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct TransactionInput {
    /// Reference to an UTXO to be spent
    pub outpoint: H256,
    /// Proof that transaction owner is authorized to spend referred UTXO &
    /// that the entire transaction is untampered
    pub sigscript: H512,
}


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
```


- `cfg_attr(feature = "std", derive(Serialize, Deserialize))`: This macro conditionally derives the `Serialize` and `Deserialize` traits when the `std` feature is enabled. This feature is useful for converting the `Transaction` struct to and from formats like JSON or other text-based formats. This functionality is often used in scenarios like data exchange between systems, debugging, or interacting with APIs or frontends.

- `Encode`, `Decode`: These macros from the parity-scale-codec crate allow the struct to be serialized to and deserialized from the SCALE binary format, ensuring efficient storage and transmission on the blockchain. Read more [SCALE](https://github.com/paritytech/parity-scale-codec).

- `TypeInfo`: Generates metadata for the struct, allowing its type information to be included in the blockchain's runtime metadata. This is valuable for interoperability with tools like Substrate frontends.


---

Configure your pallet's types, events and errors

```rust
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
    /// Read more: `runtime/src/issuance.rs`
    type Issuance: Issuance<BlockNumberFor<Self>, Value>;
}
```


---


This storage item represents the total reward value to be redistributed among authorities during block finalization.

```rust
/// Total reward value to be redistributed among authorities.
/// It is accumulated from transactions during block execution
/// and then dispersed to validators on block finalization.
#[pallet::storage]
#[pallet::getter(fn total_reward)]
pub type TotalReward<T: Config> = StorageValue<_, Value, ValueQuery>;
```

---


This storage item represents a mapping of UTXOs to their respective keys, allowing efficient lookup and management of UTXOs in a UTXO-based blockchain model.

```rust
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
```

### Events & Errors

Events and errors are used to notify about specific activity. Please use this for debugging purpose only. Events and Errors should not be used as a communication method between functionalities.
In our codebase, we will declare these errors and events. 

To declare error, simply use macro `#[pallet::error]`
```rust
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
```


To declare event, use `#[pallet::event]`. Moreover with `#[pallet::generate_deposit(pub(super) fn deposit_event)]`, it automatically generate a function `deposit_event` for emitting events.
```rust
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
```


--- 

Build the code

```sh
cargo build --release
```

--- 

Great job! 🎉 You're making fantastic progress. Let's keep the momentum going and dive into next step
