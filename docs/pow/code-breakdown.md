# Proof-of-Work Code Breakdown

> [!Important]
> Please take a look at [`sc_consensus_pow`](https://paritytech.github.io/polkadot-sdk/master/sc_consensus_pow/index.html) before dive deep into this.


Now, let's deep dive into Substrate Proof-of-Work!


## Hashing Algorithm

Firstly, you need to create **hashing algorithm**. In this repository, it use multiple hashing algorithms `MD5`, `SHA3`, `Keccak`, see [multi-pow](multi-pow/src/lib.rs).


> [Why does this use MD5, SHA3, Keccak256?](../faq.md#why-does-this-use-md5-sha3-keccak256)

**Components:**

* `Threshold`: This struct represents the difficulty level for each supported hashing algorithm (MD5, Sha3, Keccak).
```rust
pub struct Threshold {
    pub md5: U256,
    pub sha3: U256,
    pub keccak: U256,
}
```

* `SupportedHashes`: An enum that defines the supported hashing algorithms.
```rust
pub enum SupportedHashes {
    Md5,
    Sha3,
    Keccak,
}
```

* `MultiHash`: A struct that stores a hash value and the hashing algorithm used to compute it.
```rust
pub struct MultiHash {
    pub algo: SupportedHashes,
    pub value: H256,
}
```

* `Seal`: This struct represents a PoW solution, including the computed hash (`MultiHash`), difficulty level (`Threshold`), and nonce.
```rust
pub struct Seal {
    pub work: MultiHash,
    pub difficulty: Threshold,
    pub nonce: U256,
}
```

* `Compute`: This struct represents an attempt to solve the PoW puzzle before the hash is computed. It includes the difficulty level, pre-hash, and nonce.

```rust
pub struct Compute {
    pub difficulty: Threshold,
    pub pre_hash: H256,
    pub nonce: U256,
}
```

* `MultiPow`: This struct is represent for our hashing algorithm. It provides functionalities for retrieving difficulty levels and verifying PoW solutions.
```rust
pub struct MultiPow<C> {
    client: Arc<C>,
    fork_config: ForkingConfig,
}
```

* `ForkingConfig`: This enum defines different forking configurations for handling changes in supported hashing algorithms.

```rust
pub enum ForkingConfig {
    ///
    Manual,
    ///
    Automatic(ForkHeights, MaxiPosition),
}
```

* `simple_hash_meets_difficulty`: This function checks if a given hash meets the difficulty level for a specific algorithm.
* `multi_hash_meets_difficulty`: This function checks if a `MultiHash` meets the difficulty level specified in a `Threshold`.
```rust
pub fn simple_hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
    let num_hash = U256::from_little_endian(&hash[..]);
    let (_, overflowed) = num_hash.overflowing_mul(difficulty);

    !overflowed
}

pub fn multi_hash_meets_difficulty(hash: &MultiHash, difficulty: Threshold) -> bool {
    match hash.algo {
        SupportedHashes::Md5 => simple_hash_meets_difficulty(&hash.value, difficulty.md5),
        SupportedHashes::Sha3 => simple_hash_meets_difficulty(&hash.value, difficulty.sha3),
        SupportedHashes::Keccak => simple_hash_meets_difficulty(&hash.value, difficulty.keccak),
    }
}
```

* `manual_fork_validation` and `auto_fork_validation`: These functions implement the validation logic based on the forking configuration and parent block height.

```rust
/// Manual mode, the node operator manually specifies which hashing algorithms are valid through the mining client.
/// If you would like to do a fork, simply allow, un-allow some algorithms to check it.
fn manual_fork_validation(_parent_number: u32, algo: SupportedHashes) -> bool {
    use SupportedHashes::*;

    // To begin with, allow all algorithms.
    // After the fork height this check is skipped so all the hashes become valid.
    match algo {
        Md5 => true,
        Sha3 => true,
        Keccak => true,
    }
}

/// In automatic mode, the `ForkHeights` and `MaxiPosition` structs define the forking schedule
/// and the node's behavior during the contentious fork
/// (where the network splits into two chains supporting different hashing algorithms).
/// The validation logic considers the parent block height,
/// forking configuration parameters, and the hashing algorithm used in the PoW solution to determine its validity.
fn auto_fork_validation(
    parent_number: u32,
    algo: SupportedHashes,
    fork_heights: ForkHeights,
    maxi_position: MaxiPosition,
) -> bool {
    use MaxiPosition::*;
    use SupportedHashes::*;

    if parent_number < fork_heights.add_sha3_keccak {
        // To begin with we only allow md5 hashes for our pow.
        // After the fork height this check is skipped so all the hashes become valid.
        match algo {
            Md5 => true,
            Sha3 => false,
            Keccak => false,
        }
    } else if parent_number < fork_heights.remove_md5 {
        // After the first fork, all three algos become valid.
        match algo {
            Md5 => true,
            Sha3 => true,
            Keccak => true,
        }
    } else if parent_number < fork_heights.split_sha3_keccak {
        // After the second fork, md5 is no longer valid.
        match algo {
            Md5 => false,
            Sha3 => true,
            Keccak => true,
        }
    } else {
        // Finally we have the contentious fork.
        // Our behavior here depends which maxi position we have taken.
        #[allow(clippy::match_like_matches_macro)]
        match (algo, maxi_position) {
            (Sha3, Sha3Maxi) => true,
            (Sha3, NoMaxi) => true,
            (Keccak, KeccakMaxi) => true,
            (Keccak, NoMaxi) => true,
            _ => false,
        }
    }
}
```


Regarding the `sc_consensus_pow`: _"To use this engine, you can need to have a struct that implements `PowAlgorithm`. After that, pass an instance of the struct, along with other necessary client references to `import_queue` to setup the queue."_.


Hence, `MultiPow` has to implements the `PowAlgorithm` trait for the block validation and so on...


```rust
impl<B: BlockT<Hash = H256>, C> PowAlgorithm<B> for MultiPow<C>
where
    C: ProvideRuntimeApi<B>,
    C::Api: DifficultyApi<B, Threshold>,
    C: sc_client_api::HeaderBackend<B>,
{
    type Difficulty = Threshold;

    fn difficulty(&self, parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
        let difficulty = self
            .client
            .runtime_api()
            .difficulty(parent)
            .map_err(|err| {
                sc_consensus_pow::Error::Environment(format!(
                    "Fetching difficulty from runtime failed: {:?}",
                    err
                ))
            })?;

        Ok(difficulty)
    }

    fn verify(
        &self,
        parent_id: &BlockId<B>,
        pre_hash: &H256,
        pre_digest: Option<&[u8]>,
        seal: &RawSeal,
        difficulty: Self::Difficulty,
    ) -> Result<bool, Error<B>> {
        // Try to construct a seal object by decoding the raw seal given
        let seal = match Seal::decode(&mut &seal[..]) {
            Ok(seal) => seal,
            Err(_) => return Ok(false),
        };

        log::debug!("✅ Decode seal passed!");

        let Some(_encoded_pre_digest) = pre_digest else {
            return Ok(false);
        };

        log::debug!("✅ Checksum digest passed!");

        // This is where we handle forks on the verification side.
        // We will still need to handle it in the mining algorithm somewhere.
        // Currently we make the miner configure what algo they mine manually with their cli.
        let parent_number: u32 = match parent_id {
            BlockId::Hash(h) => *self
                .client
                .header(*h)
                .expect("Database should perform lookup successfully")
                .expect("parent header should be present in the db")
                .number(),
            BlockId::Number(n) => *n,
        }
        .try_into()
        .map_err(|_| ())
        .expect("Block numbers can be converted to u32 (because they are u32)");

        log::debug!("✅ Checksum parent block number passed!");

        // Here we handle the forking logic according the the node operator's request.
        let valid_algorithm = match self.fork_config {
            ForkingConfig::Manual => manual_fork_validation(parent_number, seal.work.algo),
            ForkingConfig::Automatic(fork_heights, maxi_position) => {
                auto_fork_validation(parent_number, seal.work.algo, fork_heights, maxi_position)
            }
        };

        if !valid_algorithm {
            return Ok(false);
        }

        log::debug!("✅ Valid algorithm!");

        // See whether the hash meets the difficulty requirement. If not, fail fast.
        if !multi_hash_meets_difficulty(&seal.work, difficulty) {
            return Ok(false);
        }

        log::debug!("✅ Checksum difficulty passed!");

        // Make sure the provided work actually comes from the correct pre_hash
        let compute = Compute {
            difficulty,
            pre_hash: *pre_hash,
            nonce: seal.nonce,
        };

        if compute.compute(seal.work.algo) != seal {
            return Ok(false);
        }

        log::debug!("✅ Re-compute passed!");

        log::debug!("🛠️ All passed, append the block to the chain ...");

        Ok(true)
    }
}
```


## Building PoW Runtime

### Pallet `block_author`

This pallet allows block authors to self-identify by providing an sr25519 public key.

By default, the block author is set by miner following CLI by [Inherent Transaction](https://wiki.polkadot.network/docs/learn-transactions#types-of-extrinsics). See more in `node/src/service.rs:239`


### Pallet `difficulty`

A difficulty adjustment algorithm (DAA) to keep the block time close to a particular goal
Cribbed from Kulupu https://github.com/kulupu/kulupu/blob/master/runtime/src/difficulty.rs

It is possible to implement other DAAs such as that of BTC and BCH. This would be an interesting
and worth-while experiment. The DAAs should be abstracted away with a trait.
Some ideas: https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3410460

### Trait `issuance`

A type that provides block issuance according to Bitcoin's rules. Initial issuance is _50 / block_.


Issuance is cut in half every _210,000 blocks_


Cribbed from https://github.com/Bitcoin-ABC/bitcoin-abc/blob/9c7b12e6f128a59423f4de3d6d4b5231ebe9aac2/src/validation.cpp#L1007

> [!Note]
> Approximately, 600 seconds (or 10 minutes) for a block to be finalized in Bitcoin blockchain.
> Subsidy is cut in half every 210,000 blocks which will occur approximately every 4 years.


### Pallet `utxo`

Head over to [utxo for more 😉](docs/utxo).

### Construct runtime

#### Adding pallets to the `construct_runtime!`.

Each PoW hashing algorithm is 1 pallet. Because in pallet `difficulty` I've defined it as generic pallet, so now just basically implement it with generic types.

```diff
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
+        Md5DifficultyAdjustment: difficulty::<Instance1>,
+        Sha3DifficultyAdjustment: difficulty::<Instance2>,
+        KeccakDifficultyAdjustment: difficulty::<Instance3>,
+        BlockAuthor: block_author,
+        Utxo: utxo,
    }
);
```


#### Implement `difficulty` config for runtime

> What is `Instance1`, `Instance2`, `Instance3`?

[Head over to Polkadot SDK doc - Module `frame_support::instances`](https://paritytech.github.io/polkadot-sdk/master/frame_support/instances/index.html).

`frame-support` provides some instance placeholder to be used in `frame_support::pallet` attribute macro.

```rust
...

parameter_types! {
    pub const TargetBlockTime: u128 = 5_000;
    // Setting min difficulty to damp factor per recommendation
    pub const DampFactor: u128 = 3;
    pub const ClampFactor: u128 = 2;
    pub const MaxDifficulty: u128 = u128::max_value();
}

// Helper function to get the current blocks PoW algo from the predigest
fn current_blocks_mining_algo() -> SupportedHashes {
    System::digest()
        .logs
        .iter()
        .find_map(|digest_item| match digest_item {
            DigestItem::PreRuntime(POW_ENGINE_ID, pre_digest) => {
                PreDigest::decode(&mut &pre_digest[..]).map(|d| d.1).ok()
            }
            _ => None,
        })
        .expect("There should be exactly one pow pre-digest item")
}

impl difficulty::Config<Instance1> for Runtime {
    type TimeProvider = Timestamp;
    type TargetBlockTime = TargetBlockTime;
    type DampFactor = DampFactor;
    type ClampFactor = ClampFactor;
    type MaxDifficulty = MaxDifficulty;
    type MinDifficulty = DampFactor;

    fn relevant_to_this_instance() -> bool {
        current_blocks_mining_algo() == SupportedHashes::Md5
    }
}

impl difficulty::Config<Instance2> for Runtime {
    type TimeProvider = Timestamp;
    type TargetBlockTime = TargetBlockTime;
    type DampFactor = DampFactor;
    type ClampFactor = ClampFactor;
    type MaxDifficulty = MaxDifficulty;
    type MinDifficulty = DampFactor;

    fn relevant_to_this_instance() -> bool {
        current_blocks_mining_algo() == SupportedHashes::Sha3
    }
}

impl difficulty::Config<Instance3> for Runtime {
    type TimeProvider = Timestamp;
    type TargetBlockTime = TargetBlockTime;
    type DampFactor = DampFactor;
    type ClampFactor = ClampFactor;
    type MaxDifficulty = MaxDifficulty;
    type MinDifficulty = DampFactor;

    fn relevant_to_this_instance() -> bool {
        current_blocks_mining_algo() == SupportedHashes::Keccak
    }
}
```


#### Implement `block_author` config for runtime.

Because the logic disperse block reward I've implemented on pallet utxo already, so this is not necessary. I just keep this for further building.

Feel free to change it to whatever you want like disperse block reward of utxo here instead of using hook, ...

```diff
impl block_author::Config for Runtime {
    // Each block mined issues 50 new tokens to the miner
    fn on_author_set(author_account: Self::AccountId) {
-        let issuance = 50 * TOKEN;
-        let _ = Balances::deposit_creating(&author_account, issuance);
    }
}
```

#### Implement `utxo` config for runtime.

```rust
impl utxo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // BlockAuthor pallet
    type BlockAuthor = BlockAuthor;
    // Bitcoin halving rule
    type Issuance = issuance::BitcoinHalving;
}
```

## Applying to the node

By default, when you use some Substrate template, you usually see that it use GRANDPA (GHOST-based Recursive ANcestor Deriving Prefix Agreement) for block finality.


However, in this, we are building PoW blockchain, so we have to use PoW block validation & finality. Head over to [node/service.rs](node/src/service.rs)


```rust
pub fn new_partial(
    ...
) ... {
    ...

    // A block importer for PoW.
    let block_import = sc_consensus_pow::PowBlockImport::new(
        client.clone(),
        client.clone(),
        MultiPow::new(client.clone(), fork_config),
        0, // check inherents starting at block 0
        select_chain.clone(),
        move |_, ()| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            let author =
                academy_pow_runtime::block_author::InherentDataProvider(Default::default());

            Ok((timestamp, author))
        },
    );

    // Import queue for PoW engine.
    let import_queue = sc_consensus_pow::import_queue(
        Box::new(block_import.clone()),
        None,
        MultiPow::new(client.clone(), fork_config),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    )?;
    ...
}



```rust
pub fn new_full(
    ...
) ... {
    ...
    
    // define Mining Worker
    let (mining_worker, mining_worker_task) = sc_consensus_pow::start_mining_worker(
        Box::new(pow_block_import),
        client.clone(),
        select_chain,
        MultiPow::new(client, fork_config),
        proposer,
        sync_service.clone(),
        sync_service,
        Some(PreDigest::from((sr25519_public_key.into(), mining_algo)).encode()),
        move |_, ()| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            // set default `author` following miner specified in CLI
            let author = academy_pow_runtime::block_author::InherentDataProvider(
                sr25519_public_key.encode(),
            );
            Ok((timestamp, author))
        },
        std::time::Duration::from_secs(10),
        std::time::Duration::from_secs(5),
    );

    task_manager.spawn_essential_handle().spawn_blocking(
        "pow-miner",
        Some("pow-mining"),
        mining_worker_task,
    );

    // Start Mining worker.
    use multi_pow::{multi_hash_meets_difficulty, Compute};
    use sp_core::U256;
    let mut nonce: U256 = U256::from(0);
    std::thread::spawn(move || loop {
        let worker = mining_worker.clone();
        let metadata = worker.metadata();
        if let Some(metadata) = metadata {
            let compute = Compute {
                difficulty: metadata.difficulty,
                pre_hash: metadata.pre_hash,
                nonce,
            };
            let seal = compute.compute(mining_algo);
            if multi_hash_meets_difficulty(&seal.work, seal.difficulty) {
                nonce = U256::from(0);
                let _ = futures::executor::block_on(worker.submit(seal.encode()));
            } else {
                nonce = nonce.saturating_add(U256::from(1));
                if nonce == U256::MAX {
                    nonce = U256::from(0);
                }
            }
        } else {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
```



And the rest of code in [`chain_spec.rs`](node/src/chain_spec.rs) and [`cli.rs`](node/src/cli.rs) are to start PoW node.


