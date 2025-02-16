# Interacting with UTXO

> [!Note]
> Resolve all TODO in `runtime/src/utxo.rs` to complete this step.


Now that we have established the basics of our UTXO module, let's add ways to interact with it.


### Reading Materials

Dispatchable / Call / Extrinsic functions:

- [Polkadot Academy Book - FRAME Call](https://polkadot-blockchain-academy.github.io/pba-book/frame/calls/page.html)

> [!Note]
> Note that we'll implement UTXO pallet in `dev_mode`, it is not necessary to specify `weight` and `call_index`. However, feel free to do it if you want.


### Implement for Pallet UTXO 

Implement `spend` extrinsic. Highly recommend that keep the function simple and move all logics to intrinsics named with prefix `do_...`, `execute_...` (or whatever prefix you want 👀).

```rust
pub fn spend(_origin: OriginFor<T>, transaction: Transaction) -> DispatchResult {
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
```

---

Strips a transaction of its Signature fields by replacing value with ZERO-initialized fixed hash.


Just is for demo usage, feel free to add logic of using additional salt yourself 😉.

```rust
fn get_simple_transaction(transaction: &Transaction) -> Vec<u8> {
    let mut trx = transaction.clone();
    for input in trx.inputs.iter_mut() {
        input.sigscript = H512::zero();
    }

    trx.encode()
}
```

---

Implement validate_transaction intrinsic with criteria:

- [ ] Inputs and outputs are not empty
- [ ] Each input is used exactly once
- [ ] All inputs match to existing, unspent and unlocked outputs
- [ ] Sum of input values does not overflow
- [ ] Each output is defined exactly once and has nonzero value
- [ ] Sum of output values does not overflow
- [ ] Total output value must not exceed total input value

```rust
/// Check transaction for validity, errors, & race conditions
/// Called by both transaction pool and runtime execution
pub fn validate_transaction(
    transaction: &Transaction,
) -> Result<ValidTransaction, &'static str> {
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
```

---

Implement `do_spend` intrinsic for `spend` extrinsic

```rust
/// Implement spend logic, update storage to reflect changes made by transaction
/// Where each UTXO key is a hash of the entire transaction and its order in the `TransactionOutputs` vector
fn do_spend(transaction: &Transaction, reward: Value) -> DispatchResult {
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
        // Optional, this event I used for log 
        Self::deposit_event(Event::TransactionOutputProcessed(hash));
    }

    Ok(())
}
```

---

Implement `store_utxo` for storage mutation

```rust
 fn store_utxo(utxo: &TransactionOutput, hash: H256) {
    // TODO [3-spend-utxo]
    UtxoStore::<T>::insert(hash, utxo);
    
    // further update 😉
}
```

--- 

Build the code

```sh
cargo build --release
```

--- 

Awesome work completing this step! 🚀 You're doing great. Now, let's advance to next step.
