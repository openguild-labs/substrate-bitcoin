# Initialize Working Environment

In this step, we will initialize a basic Javascript project, where we can start building our simple client script.

Make sure you have installed [Node.js](https://nodejs.org/en) in your machine.

## NPM init

1. Create a new directory for your project:

```sh
mkdir scripts/generate-signature
cd scripts/generate-signature
```

2. In that folder, run the following command to initialize a new Node.js project:

```sh
npm init -y
```

You'll see the `package.json` file created in your project directory. This file contains metadata about your project, such as its name, version, description, author, and dependencies.

3. Install necessary dependencies

```sh
npm install @polkadot/api @polkadot/keyring @polkadot/types @polkadot/util @polkadot/wasm-crypto
```
