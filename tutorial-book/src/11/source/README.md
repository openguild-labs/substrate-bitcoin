# Creating script

We'll create a light weight client for the Polkadot network. This script will allow us to interact with the Polkadot network using the Polkadot API.

## Start local node:

At the root of the project, run the following command to start a local node:
```sh
./target/release/academy-pow --dev --tmp
```

## Implement script

1. Create `scripts/generate-signature/index.js` file.

```sh
cd scripts/generate-signature
touch index.js
```

2. Create client
```javascript
const { WsProvider } = require('@polkadot/api');

const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Replace with your endpoint

async function main() {
}

main().then(() => process.exit(0)).catch(error => {
    console.error(error);
    process.exit(1);
});
```

3. Define keyring and data structure

```diff
const { WsProvider } = require('@polkadot/api');
+const Keyring = require('@polkadot/keyring').default;
+const { ApiPromise } = require('@polkadot/api');

const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Replace with your endpoint

async function main() {
+    // because our account address is in `sr25519` format
+    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
+    // Register types
+    const api = await ApiPromise.create({
+        types: {
+            TransactionInput: {
+                outpoint: 'H256',      // Fixed 32 bytes
+                sigscript: 'H512'     // Fixed 64 bytes
+            },
+            TransactionOutput: {
+                value: 'u128',        // 16 bytes
+                pubkey: 'H256'        // Fixed 32 bytes
+            },
+            Transaction: {
+                inputs: 'Vec<TransactionInput>',  // Vec type
+                outputs: 'Vec<TransactionOutput>' // Vec type
+            }
+        },
+        provider: wsProvider, // Add the provider here
    });
}

main().then(() => process.exit(0)).catch(error => {
    console.error(error);
    process.exit(1);
});
```

4. Define signer

```javascript
async function main() {
    // ...

    // create Alice based on the development seed. You can change to any other signer if you want
    const alice = keyring.addFromUri('//Alice');
}
```

5. Define transaction data

**Scenarios:**
- Alice address in sr25519 format: `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`.
- Bob address in sr25519 format: `0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48`.
- Alice has an UTXO `0xc670c5f69081da78af400552edcafa3f0f31e84db1b50dd70776e0f87477b3dc` worth 100.
- Alice wants to send 50 to Bob.
- Alice gets back 50.


```javascript
async function main() {
    // ...

    const inputs = [{
        // the latest UTXO hash of the account want to spent
        outpoint: "0xc670c5f69081da78af400552edcafa3f0f31e84db1b50dd70776e0f87477b3dc",
        // default simple sigscript
        sigscript: "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    }]
    const outputs = [
        {
            // Value to spend
            value: "50",
            // Bob pubkey
            pubkey: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        },
        {
            // Value to spend
            value: "50",
            // Alice pubkey
            pubkey: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        },
    ]
}
```

6. Encode data then sign the data

```javascript
// ...
const { u8aToHex } = require('@polkadot/util');

async function main() {
    // ...

    // Encode full transaction
    const encodedTx = api.createType('Transaction', {
        inputs: inputs,
        outputs: outputs
    }).toU8a();

    // the encoded transaction in u8 array
    const signature = alice.sign(encodedTx);
    const isValid = alice.verify(encodedTx, signature, alice.publicKey);

    if (isValid) {
        console.log(`✅ Signature: ${u8aToHex(signature)}`);
    } else {
        console.log(`❌ Something went wrong! Please report to 👉 https://github.com/danielbui12/substrate-bitcoin-like-blockchain/issues/new`);
    }
}
```

7. Run the script

```sh
node index.js

2025-02-10 15:08:18        API/INIT: RPC methods not decorated: chainHead_v1_body, chainHead_v1_call, chainHead_v1_continue, chainHead_v1_follow, chainHead_v1_header, chainHead_v1_stopOperation, chainHead_v1_storage, chainHead_v1_unfollow, chainHead_v1_unpin, transactionWatch_v1_submitAndWatch, transactionWatch_v1_unwatch, transaction_v1_broadcast, transaction_v1_stop
✅ Signature: 0xe6d4b51e3a892a074eff4e1d7358b76c70c36f7956fb1c203d7c8e8504a51f469c7ce771646f7dd751cee91051b46b52508edfba7b318004a429ee7b255bbe8d

```

