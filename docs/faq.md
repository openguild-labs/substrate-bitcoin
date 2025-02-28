# FAQ

### Why does this use MD5, SHA3, Keccak256?

Basically they're more simple & faster than SHA256, good to demonstrates legacy hashing mechanisms. However, MD5 is insecure due to vulnerabilities like collision attacks, making it unsuitable for real-world blockchain applications.


If you get this well, you can add SHA256 to your own Proof-of-Work


### How do I get `sigscript`?

Sigscript is signed data from _Alice_. Look at `runtime/src/utxo.rs:346` and `scripts/generate-signature/generate-signature.js:14` to explore how it works.

### How does it maintain balance of wallet?

Every time a wallet makes transfer UTXO, the `input(s) UTXO` will be removed, replaced for `outputs UTXO`. Please read the [scenario](demo-usage.md).

In this case:
- Alice UTXO initial balance was removed
- After invoke `spend` extrinsic, state contains 2 new UTXOs, 1 for Alice rest UTXO balance, 1 for Bob new UTXO balance.
- Absolutely, the logic does verify that `total output spend` must be less than `total input spend`




