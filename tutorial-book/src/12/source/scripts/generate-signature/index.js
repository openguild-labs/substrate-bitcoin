const { WsProvider } = require('@polkadot/api');
const Keyring = require('@polkadot/keyring').default;
const { ApiPromise } = require('@polkadot/api');
const { u8aToHex } = require('@polkadot/util');

const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Replace with your endpoint

async function main() {
    // because our account address is in `sr25519` format
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    // Register types
    const api = await ApiPromise.create({
        types: {
            TransactionInput: {
                outpoint: 'H256',      // Fixed 32 bytes
                sigscript: 'H512'     // Fixed 64 bytes
            },
            TransactionOutput: {
                value: 'u128',        // 16 bytes
                pubkey: 'H256'        // Fixed 32 bytes
            },
            Transaction: {
                inputs: 'Vec<TransactionInput>',  // Vec type
                outputs: 'Vec<TransactionOutput>' // Vec type
            }
        },
        provider: wsProvider, // Add the provider here
    });

    // create Alice based on the development seed. You can change to any other signer if you want
    const alice = keyring.addFromUri('//Alice');

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

main().then(() => process.exit(0)).catch(error => {
    console.error(error);
    process.exit(1);
});