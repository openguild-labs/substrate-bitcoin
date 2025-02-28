# Initialize your project

## Clone the repository

```sh
git clone -b start_here https://github.com/danielbui12/substrate-bitcoin-like-blockchain
```

## Build your project

Let's build your project to verify that your new project is working as expected by running:

```sh
cd substrate-bitcoin-like-blockchain
cargo build --release
```

After build success, please run the node by following command:

```sh
❯ ./target/release/academy-pow --dev --tmp
2025-02-10 10:34:04 Academy PoW Chain    
2025-02-10 10:34:04 ✌️  version 3.0.0-3289a2a2d00    
2025-02-10 10:34:04 ❤️  by @danielbui12, 2025-2025    
2025-02-10 10:34:04 📋 Chain specification: Development    
2025-02-10 10:34:04 🏷  Node name: broken-class-8644    
2025-02-10 10:34:04 👤 Role: AUTHORITY    
2025-02-10 10:34:04 💾 Database: ParityDb at /var/folders/n1/4b940c8s1qddng2m8xgxwb280000gn/T/substratea5r5zS/chains/dev/paritydb/full    
2025-02-10 10:34:04 🔨 Initializing Genesis block/state (state: 0x6118…d3ad, header-hash: 0xad3e…d8b4)    
2025-02-10 10:34:04 Using default protocol ID "sup" because none is configured in the chain specs    
2025-02-10 10:34:04 Local node identity is: 12D3KooWQjFVngak2ZeaiTkBonVp5Ju69HGEowLubf7VFWyctFDC    
2025-02-10 10:34:04 Running litep2p network backend    
2025-02-10 10:34:04 💻 Operating system: macos    
2025-02-10 10:34:04 💻 CPU architecture: aarch64    
2025-02-10 10:34:04 📦 Highest known block at #0    
2025-02-10 10:34:04 〽️ Prometheus exporter started at 127.0.0.1:9615    
2025-02-10 10:34:04 Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["*"]    
2025-02-10 10:34:09 💤 Idle (0 peers), best: #0 (0xad3e…d8b4), finalized #0 (0xad3e…d8b4), ⬇ 0 ⬆ 0    
2025-02-10 10:34:14 💤 Idle (0 peers), best: #0 (0xad3e…d8b4), finalized #0 (0xad3e…d8b4), ⬇ 0 ⬆ 0    
2025-02-10 10:34:14 🙌 Starting consensus session on top of parent 0xad3e45470dd74feceeecf2e11a9bb18ed97c813c53af0160baea0b81d23fd8b4 (#0)    
2025-02-10 10:34:14 🎁 Prepared block for proposing at 1 (2 ms) [hash: 0x4544f634084a6ffad3b6fa3727ad5710bf28337851b003b4d3d14844e7e33ab8; parent_hash: 0xad3e…d8b4; extrinsics (2): [0x1eb2…66b0, 0x6591…3bcd]    
2025-02-10 10:34:16 ✅ Successfully mined block on top of: 0xad3e…d8b4    
2025-02-10 10:34:16 🏆 Imported #1 (0xad3e…d8b4 → 0xe8ee…1ced)    
2025-02-10 10:34:16 🙌 Starting consensus session on top of parent 0xe8eefcc3db4ef32c68907833966ee0fc128098de624b5589409dc0a50dd41ced (#1)    
2025-02-10 10:34:16 🎁 Prepared block for proposing at 2 (0 ms) [hash: 0x05b04b1396c7498daf210e8aa512ee1878e083e38b43ddac12be7162e28e6cea; parent_hash: 0xe8ee…1ced; extrinsics (2): [0x132e…94eb, 0x6591…3bcd]    
2025-02-10 10:34:19 ✅ Successfully mined block on top of: 0xe8ee…1ced    
2025-02-10 10:34:19 🏆 Imported #2 (0xe8ee…1ced → 0x4481…c80e)    
2025-02-10 10:34:19 🙌 Starting consensus session on top of parent 0x44812abaf4045df92c9708468fa043274bbf94bfd951c20805411cef23b6c80e (#2)    
2025-02-10 10:34:19 🎁 Prepared block for proposing at 3 (0 ms) [hash: 0x04f45b59da4afd41d800d2b24edbaa607d71ab7ee153b92abe6ab86d1d822284; parent_hash: 0x4481…c80e; extrinsics (2): [0x82b2…48d4, 0x6591…3bcd]    
2025-02-10 10:34:19 💤 Idle (0 peers), best: #2 (0x4481…c80e), finalized #0 (0xad3e…d8b4), ⬇ 0 ⬆ 0
```

✅   If you see the above output, then your project is ready to be used. You can now proceed to the next step.