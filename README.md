# Partnerchain reference implementation

This repository will contain a reference implementation of a Substrate partnerchain that can be used by developers in the ecosystem as blueprint / example to build their own specific chains. The project involves building a fully functional partnerchain within the context of a particular use-case.

## Table of Contents

- [Structure](#structure)

- [Getting Started](#getting-started)

- [Starting a Minimal Template Chain](#starting-a-minimal-template-chain)

  - [Omni Node](#omni-node)
  - [Minimal Template Node](#minimal-template-node)
  - [Connect with the Polkadot-JS Apps Front-End](#connect-with-the-polkadot-js-apps-front-end)

## Structure

A Polkadot SDK based project such as this one consists of:

- 🧮 the [Runtime](./runtime/README.md) - the core logic of the blockchain.
- 🎨 the [Pallets](./pallets/README.md) - from which the runtime is constructed.
- 💿 a [Node](./node/README.md) - the binary application (which is not part of the cargo default-members list and is not
compiled unless building the entire workspace).

## Getting Started

- 👉 Check the
[Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

- 🛠️ Depending on your operating system and Rust version, there might be additional
packages required to compile this template - please take note of the Rust compiler output.

Usually, it will be necessary to add the `wasm32-unknown-unknown` target, and the `rust-src` component, both of which can be installed, for example in Linux by executing the following commands:

```sh
$ rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu
$ rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
```

- Fetch minimal template code.

## Starting a Minimal Template Chain

### Omni Node

[Omni Node](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html) can
be used to run the minimal template's runtime. `polkadot-omni-node` binary crate usage is described at a high-level
[on crates.io](https://crates.io/crates/polkadot-omni-node).

#### Install `polkadot-omni-node`

Please see installation section on [crates.io/omni-node](https://crates.io/crates/polkadot-omni-node).

#### Install `staging-chain-spec-builder`

Please see the installation section at [`crates.io/staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder).

#### Build

```sh
cargo build
```

#### Use chain-spec-builder to generate the chain_spec.json file for a custom runtime

The project includes a [`chain_spec`](dev_chain_spec.json) file to test initially. But if you want to test out a custom runtime you can use the `chain-spec-builder` command like the following to create the file:

```sh
chain-spec-builder create --relay-chain "dev" --para-id 1000 --runtime \
    target/debug/wbuild/minimal-template-runtime/minimal_template_runtime.wasm named-preset development
```

**Note**: the `relay-chain` and `para-id` flags are extra bits of information required to
configure the node for the case of representing a parachain that is connected to a relay chain.
They are not relevant to minimal template business logic, but they are mandatory information for
Omni Node, nonetheless.

#### Run Omni Node

Start Omni Node in development mode (sets up block production and finalization based on manual seal,
sealing a new block every 3 seconds), with a minimal template runtime chain spec.

```sh
polkadot-omni-node --chain <path/to/chain_spec.json> --dev
```

### Minimal Template Node

#### Build both node & runtime

🐙 Alternatively, you can build all of the artifacts like the runtime and the node:

```sh
cargo build --workspace --release
```

To run use:

```sh
<target/release/path/to/minimal-template-node> --chain <path/to/chain_spec.json> --tmp --consensus manual-seal-3000
```

#### Build and run with docker

🐳 Build the docker image which builds all the workspace members, creates the chain specification, and has as entry point the node binary:

```sh
docker build . -t polkadot-sdk-minimal-template
```
Then run using:

```sh
docker run polkadot-sdk-minimal-template --chain polkadot/chain_spec.json --base-path /data
```

### Connect with the Polkadot-JS Apps Front-End

- 🌐 You can interact with your local node using the
hosted version of the [Polkadot/Substrate
Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944).
