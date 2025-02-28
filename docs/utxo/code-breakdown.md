# UTXO Pallet Code Breakdown

## Table of Contents

- [Struct Definitions](#struct-definitions)
  - [Transaction](#transaction)
  - [TransactionInput](#transactioninput)
  - [TransactionOutput](#transactionoutput)
- [Storage](#storage)
- [Genesis Configuration](#genesis-configuration)
- [Events and Errors](#events-and-errors)
- [Hooks](#hooks)
- [Dispatchable Functions](#dispatchable-functions)
- [Helper Methods](#helper-methods)

---

## Struct Definitions

> As defined at [UTXO](utxo.md), so this will not go detail each structure again.

### Transaction
Represents a single transaction with inputs and outputs.
```rust
/// Single transaction to be dispatched
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct Transaction {
	/// UTXOs to be used as inputs for current transaction
	pub inputs: Vec<TransactionInput>,

	/// UTXOs to be created as a result of current transaction dispatch
	pub outputs: Vec<TransactionOutput>,
}
```
- **Inputs**: References existing UTXOs.
- **Outputs**: Creates new UTXOs as a result of the transaction.

### TransactionInput
Represents a single input referring to a UTXO.
```rust
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
```
- **outpoint**: A hash referring to a UTXO.
- **sigscript**: A cryptographic proof of ownership.

### TransactionOutput
Defines the value and ownership of an output.
```rust
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
```
- **value**: The amount associated with the output.
- **pubkey**: The public key of the owner.

---

## Storage
### TotalReward
```rust
/// Total reward value to be redistributed among authorities.
/// It is accumulated from transactions during block execution
/// and then dispersed to validators on block finalization.
#[pallet::storage]
#[pallet::getter(fn total_reward)]
pub type TotalReward<T: Config> =
    StorageValue<_, Value, ValueQuery>;
```
Stores the accumulated rewards for the block author.

If your `total_input_value > total_output_value`, the `total_input_value - total_output_value` will be used as `reward` aka `priority fee`.

```rust
    if missing_utxos.is_empty() {
        ensure!( total_input >= total_output, Error::<T>::OutputOverInput);
        reward = total_input.checked_sub(total_output).ok_or(Error::<T>::RewardOverflow)?;
    }
```

Here is a use case, Alice has an UTXO $100, and transfers to Bob an UTXO $50, and get back only UTXO $10.

In `runtime/src/issuance.rs:30`, defined initial reward fee per block is `const INITIAL_ISSUANCE: u32 = 50;`

Hence, total reward is 50 + 40 = 90!

![example_tips](assets/example_tips.png)

### UtxoStore
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
        Key=H256,
        Value=TransactionOutput,
        QueryKind=OptionQuery
    >;
```
Stores all valid UTXOs, mapping their hash to the output details.

### UtxoOf
```rust
#[pallet::storage]
/// Keep track of latest UTXO hash of account
/// Mapping from `sr25519::Pubkey` to `BlakeTwo256::hash_of(transaction, index)`
#[pallet::storage]
#[pallet::getter(fn utxo_of)]
pub type UtxoOf<T: Config> = StorageMap<
        Hasher = Identity,
        Key=Public,
        Value=H256,
        QueryKind=OptionQuery
    >;
```
Tracks the latest UTXO hash for each account.

---

## Genesis Configuration
### GenesisConfig
Defines the initial state of the UTXO system.
```rust
pub struct GenesisConfig<T: Config> {
    pub _ph_data: PhantomData<T>,
    pub genesis_utxos: Vec<TransactionOutput>,
}
```
- **genesis_utxos**: Initial UTXOs for system bootstrap.

### BuildGenesisConfig
Populates storage with the initial UTXOs.
```rust
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
    fn build(&self) {
        for utxo in self.genesis_utxos.iter() {
            let hash = BlakeTwo256::hash_of(&utxo);
            Pallet::<T>::store_utxo(&utxo, hash);
        }
    }
}
```

---

## Events and Errors
### Events
Notifies users about significant state changes.
```rust
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
    RewardWasted
}
```
### Errors
Handles runtime errors for transaction validation.
```rust
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
```

---

## Hooks
Executes logic at specific block lifecycle stages.
```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_finalize(_n: BlockNumberFor<T>) {
        match T::BlockAuthor::block_author() {
            None => Self::deposit_event(Event::RewardWasted),
            Some(author) => Self::disperse_reward(&author),
        }
    }
}
```
- **on_finalize**: Distributes rewards to the block author or logs waste.



---

## Dispatchable Functions
### spend
Processes a transaction.
```rust
pub fn spend(_origin: OriginFor<T>, transaction: Transaction) -> DispatchResult {
    // validate transaction, got `TransactionValidity`
    let transaction_validity = Self::validate_transaction(&transaction)?;
    // make sure transaction is not missing anything
    ensure!(transaction_validity.requires.is_empty(), Error::<T>::MissingInput);
    // update storage
    Self::update_storage(&transaction, transaction_validity.priority as Value)?;
    // emit event
    Self::deposit_event(Event::<T>::TransactionSuccess(transaction));
    Ok(())
}
```

### faucet
Issues funds to an account.
```rust
pub fn faucet(_origin: OriginFor<T>, to: Public, value: Value) -> DispatchResult {
    let _ = Self::deposit_creating(&to, value);
    Ok(())
}
```

---

## Helper Methods
### update_storage
Updates storage for processed transactions.
```rust
fn update_storage(transaction: &Transaction, reward: Value) -> DispatchResult {
    // check if transaction includes tip, then add to `TotalReward` for miner can claim
    let new_total = TotalReward::<T>::get().checked_add(reward).ok_or(Error::<T>::RewardOverflow)?;
    TotalReward::<T>::put(new_total);

    // remove current `TransactionInput`
    for input in &transaction.inputs {
        UtxoStore::<T>::remove(input.outpoint);
    }

    for output in &transaction.outputs {
        let hash = BlakeTwo256::hash_of(&(&transaction.encode(), index));
        // add new `TransactionOutput`
        Self::store_utxo(output, hash);
    }
    Ok(())
}
```

### disperse_reward
Distributes rewards to the block author.
```rust
fn disperse_reward(author: &Public) {
    // get the total reward, and reset instantly afterward
    let reward = TotalReward::<T>::take() + T::Issuance::issuance(
        frame_system::Pallet::<T>::block_number()
    );
    let utxo = TransactionOutput { value: reward, pubkey: H256::from_slice(author.as_slice()) };
    let hash = BlakeTwo256::hash_of(&utxo);
    Self::store_utxo(&utxo, hash);
    Self::deposit_event(Event::RewardDistributed(reward, hash));
}
```

### store_utxo 
Inner function helps update state
```rust
fn store_utxo(utxo: &TransactionOutput, hash: H256) {
    // insert latest `TransactionOutput`
    UtxoStore::<T>::insert(hash, utxo);
    // Convert H256 back to sr25519::Public
    let pubkey = Public::from_h256(utxo.pubkey);
    log::info!("pubkey: {:?}", pubkey);
    // insert latest hash of transaction for easily retrieving
    UtxoOf::<T>::insert(pubkey, hash);
}
```

### validate_transaction
Validate transaction, return the `TransactionValidity`
```rust
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
pub fn validate_transaction(transaction: &Transaction) -> Result<ValidTransaction, &'static str> {
    // Check basic requirements
    ensure!(!transaction.inputs.is_empty(), Error::<T>::EmptyInput);
    ensure!(!transaction.outputs.is_empty(), Error::<T>::EmptyOutput);

    {
        let input_set: BTreeMap<_, ()> = transaction.inputs.iter().map(|input| (input, ())).collect();
        ensure!(input_set.len() == transaction.inputs.len(), Error::<T>::DuplicatedInput);
    }
    {
        let output_set: BTreeMap<_, ()> = transaction.outputs.iter().map(|output| (output, ())).collect();
        ensure!(output_set.len() == transaction.outputs.len(), Error::<T>::DuplicatedOutput);
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
                    &Public::from_h256(input_utxo.pubkey)
            );
            log::info!("is_valid_sig: {:?}", is_valid_sig);
            ensure!(is_valid_sig, Error::<T>::InvalidSignature);
            total_input = total_input.checked_add(input_utxo.value).ok_or(Error::<T>::InputOverflow)?;
        } else {
            missing_utxos.push(input.outpoint.clone().as_fixed_bytes().to_vec());
        }
    }

    // Check that outputs are valid
    for output in transaction.outputs.iter() {
        ensure!(output.value > 0, Error::<T>::ZeroAmount);
        let hash = BlakeTwo256::hash_of(&(&transaction.encode(), output_index));
        output_index = output_index.checked_add(1).ok_or(Error::<T>::MaximumTransactionDepth)?;
        ensure!(!UtxoStore::<T>::contains_key(hash), Error::<T>::DuplicatedOutput);
        total_output = total_output.checked_add(output.value).ok_or(Error::<T>::OutputOverflow)?;
        new_utxos.push(hash.as_fixed_bytes().to_vec());
    }

    // If no race condition, check the math
    if missing_utxos.is_empty() {
        ensure!( total_input >= total_output, Error::<T>::OutputOverInput);
        reward = total_input.checked_sub(total_output).ok_or(Error::<T>::RewardOverflow)?;
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
```

> Why does it return `TransactionValidity` instead of `Result`?

Look at `runtime/lib.rs:441`, we'll pre-validate transaction, and if it pass, it will not use `Executive::validate_transaction`, that's why we can submit unsigned transaction, no need to pay any fee of `pallet_balances`.
```rust
impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(
        source: TransactionSource,
        tx: <Block as BlockT>::Extrinsic,
        block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
        // Extrinsics representing UTXO transaction need some special handling
        if let Some(&utxo::Call::spend{ ref transaction }) = IsSubType::<<Utxo as Callable<Runtime>>::RuntimeCall>::is_sub_type(&tx.function) 
        {
            match Utxo::validate_transaction(&transaction) {
                // Transaction verification failed
                Err(e) => {
                    sp_runtime::print(e);
                    return Err(TransactionValidityError::Invalid(InvalidTransaction::Custom(1)));
                }
                // Race condition, or Transaction is good to go
                Ok(tv) => { return Ok(tv); }
            }
        }

        // Fall back to default logic for non UTXO-spending extrinsics
        Executive::validate_transaction(source, tx, block_hash)
    }
}
```
