# Runtime

> [!Note]
> Resolve all TODO in `runtime/src/lib.rs` to complete this step.

### Reading Materials

- [Polkadot Academy Book - Substrate's Transaction Pool](https://polkadot-blockchain-academy.github.io/pba-book/substrate/txpool-api/page.html)
- [Polkadot Academy Book - `construct_runtime!`](https://polkadot-blockchain-academy.github.io/pba-book/frame/construct-runtime/page.html)


### Implement

From the first step of building environment, I've already import pallet into the runtime for ya

```rust
/// UTXOs serve as the digital equivalent of change you receive after making a cash purchase
pub mod utxo;
```

Now, you just have to add the pallet UTXO to the runtime

```diff
+ impl utxo::Config for Runtime {
+    type RuntimeEvent = RuntimeEvent;
+    type BlockAuthor = BlockAuthor;
+    type Issuance = issuance::BitcoinHalving;
+ }

... 

construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Md5DifficultyAdjustment: difficulty::<Instance1>,
        Sha3DifficultyAdjustment: difficulty::<Instance2>,
        KeccakDifficultyAdjustment: difficulty::<Instance3>,
        BlockAuthor: block_author,
+        Utxo: utxo,
    }
);

...



impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(
        source: TransactionSource,
        tx: <Block as BlockT>::Extrinsic,
        block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
+        // Extrinsics representing UTXO transaction need some special handling
+        if let Some(&utxo::Call::spend{ ref transaction }) = IsSubType::<<Utxo as Callable<Runtime>>::RuntimeCall>::is_sub_type(&tx.function)
+        {
+            match Utxo::validate_transaction(&transaction) {
+                // Transaction verification failed
+                Err(e) => {
+                    sp_runtime::print(e);
+                    return Err(TransactionValidityError::Invalid(InvalidTransaction::Custom(1)));
+                }
+                // Race condition, or Transaction is good to go
+                Ok(tv) => { return Ok(tv); }
+            }
+        }

        // Fall back to default logic for non UTXO-spending extrinsics
        Executive::validate_transaction(source, tx, block_hash)
    }
}


```

--- 

Build the code

```sh
cargo build --release
# start temporary local node in development environment
./target/release/academy-pow --dev --tmp
```

Direct to [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), and see block production. If you see after every a new block, there's also an event `RewardDistributed`, your work are perfect!

![result](https://github.com/danielbui12/substrate-bitcoin-like-blockchain/blob/step-5-runtime/docs/assets/explorer_pow_reward.png?raw=true)

--- 

Well done on completing! 🎉 You're on fire! Now, let's tackle next step.
