# Genesis Builder

> [!Note]
> Resolve all TODO in `runtime/src/utxo.rs` and `runtime/src/chain_spec.rs` to complete this step.

### Reading Materials

We've almost complete the UTXO pallet. However we have no balance to test it, that's quite ridiculous 🫤.

In this tutorial, I'll show you how to add gensis data for pallet, have initial UTXO balances, and something to easier to test.

Moreover, you can add `faucet` extrinsic to pallet, that's work too. I let this is challenge for you. Feel free to complete it. The solution is on [main branch](https://github.com/danielbui12/substrate-bitcoin-like-blockchain/blob/main/runtime/src/utxo.rs#L224).


- [Polkadot Academy Book - Genesis and Builder](https://polkadot-blockchain-academy.github.io/pba-book/frame/construct-runtime/page.html#testing-genesis-and-builder)


### Implement

Firstly, we've to declare genesis data type.

> [!Important]
> Because code is built on `no-std` feature. If we keep using the `TransactionOutput` struct, we'll get error 
> 
> ... "the trait `Serialize` is not implemented for `TransactionOutput`" ...
> 
> ... "the trait `Deserialize<'_>` is not implemented for `TransactionOutput`" ...
> 
> Hence, we need to simplify data type to make it work in both `std` and `no-std` feature.

```rust
...
/// Genesis Utxo Type
pub type GenesisUtxoType = (Value, H256);

...
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub _ph_data: Option<PhantomData<T>>,
    pub genesis_utxos: Vec<GenesisUtxoType>,
}
``` 

---

Implement `Default` trait for `GenesisConfig`:

```rust
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
        Self {
            _ph_data: Default::default(),
            genesis_utxos: Vec::<GenesisUtxoType>::new(),
        }
    }
}
```

---

Implement logic to build genesis data

```rust
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
```

---

Additionally, I've put some codes to make it easier for testing as mentioned above

```rust
...

/// Keep track of latest UTXO hash of account
/// Mapping from `sr25519::Pubkey` to `BlakeTwo256::hash_of(transaction, index)`
/// Just for testing 🫤
/// Because 1 account may have multiple UTXOs
#[pallet::storage]
#[pallet::getter(fn utxo_of)]
pub type UtxoOf<T: Config> =
    StorageMap<Hasher = Identity, Key = Public, Value = H256, QueryKind = OptionQuery>;

...

fn store_utxo(utxo: &TransactionOutput, hash: H256) {
    ...    
    // Convert H256 back to sr25519::Public
    let pubkey = Public::from_h256(utxo.pubkey);
    UtxoOf::<T>::insert(pubkey, hash);
}

...
```

---

Move on the `node/src/chain_spec.rs`, add some genesis data for pallet UTXO:

```diff

...

use academy_pow_runtime::{
    AccountId,
    SS58Prefix,
    Signature,
    TOKEN_DECIMALS,
    TOKEN_SYMBOL,
    WASM_BINARY,
+   utxo::{GenesisUtxoType, Value},
};

...

pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        ForkingExtensions {
            manual_mode: 1, // change this to `0` if you want to try `auto_fork_validation`
            add_sha3_keccak: 10,
            remove_md5: 20,
            split_sha3_keccak: 30,
            maxi_position: String::from("follow-mining"),
        },
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(genesis(
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        ],
        // Initial Difficulty
        4_000_000,
+       vec![
+           get_account_id_from_seed::<sr25519::Public>("Alice"),
+           get_account_id_from_seed::<sr25519::Public>("Bob"),
+           get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
+           get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
+       ],
    ))
    .with_properties(system_properties())
    .build())
}

...


pub fn testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        ForkingExtensions {
            manual_mode: 1,
            add_sha3_keccak: 0,
            remove_md5: 0,
            split_sha3_keccak: 0,
            maxi_position: String::new(),
        },
    )
    .with_name("Testnet")
    .with_id("testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(genesis(
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        ],
        4_000_000,
+        vec![
+            get_account_id_from_seed::<sr25519::Public>("Alice"),
+            get_account_id_from_seed::<sr25519::Public>("Bob"),
+            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
+            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
+        ],
    ))
    .with_properties(system_properties())
    .build())
}

...



fn genesis(
    endowed_accounts: Vec<AccountId>,
    initial_difficulty: u32,
+   utxo_genesis_accounts: Vec<AccountId>,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            // Configure endowed accounts with initial balance of 1 << 50.
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 50)).collect::<Vec<_>>(),
        },
        "keccakDifficultyAdjustment": {
            "initialDifficulty": u32_to_u8_32(initial_difficulty),
        },
        "md5DifficultyAdjustment": {
            "initialDifficulty": u32_to_u8_32(initial_difficulty),
        },
        "sha3DifficultyAdjustment": {
            "initialDifficulty": u32_to_u8_32(initial_difficulty),
        },
+         "utxo": {
+             "genesisUtxos": utxo_genesis_accounts
+                 .iter().cloned()
+                 .map(|k| {
+                     let hash = H256::from_slice(&k.as_slice()); 
+                     let value: Value = (1u64 << 50).into();
+                     let genesis_utxo: GenesisUtxoType = (value, hash);
+                     genesis_utxo
+                 }).collect::<Vec<GenesisUtxoType>>(),
+         },
    })
}

...

```

We've add some balances for Alice, Bob, also hard-derive account of Alice and Bob. Let's testing.


--- 

Build the code

```sh
cargo build --release
# start temporary local node in development environment
./target/release/academy-pow --dev --tmp
```

> [!Note]
> Because Substrate default account is in SS58 format, so we've to map that address to sr25519 format.
> Powerful tool to convert `SS58 account` to `sr25519::Pubkey`: https://polkadot.subscan.io/tools/format_transform

- Alice: from `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` to `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`
- Bob: from `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty` to `0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48` 


Direct to [https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/chainstate](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/chainstate).


Select pallet UTXO > `utxoOf`:
- Input: `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`, press `+` button.
- Output: 
```
utxo.utxoOf: Option<H256>
0xc670c5f69081da78af400552edcafa3f0f31e84db1b50dd70776e0f87477b3dc`
```

Select pallet UTXO > `utxoStore`:
- Input: `0xc670c5f69081da78af400552edcafa3f0f31e84db1b50dd70776e0f87477b3dc`, press `+` button.
- Output: 
```
utxo.utxoStore: Option<AcademyPowRuntimeUtxoTransactionOutput>
{
  value: 1,125,899,906,842,624
  pubkey: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
}
```

--- 

Great work completing a new step! 💪 You're almost at the finish line. Now, let's dive into last step. Run this command to continue:  
