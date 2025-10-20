# Partnerchain reference implementation

This repository will contain a reference implementation of a Substrate partnerchain that can be used by developers in the ecosystem as blueprint / example to build their own specific chains. The project involves building a fully functional partnerchain within the context of a particular use-case.

## Table of Contents

- [Documentation](#documentation)

- [Getting Started](#getting-started)

- [Starting a chain](#starting-a-local-chain)

  - [Local chain](#build-your-artifacts)

  - [Start a chain with docker](#build-and-run-with-docker)

## Documentation

In the [docs](./docs/) folder you can find important information such as the [contribution guidelines](./docs/CONTRIBUTING.md) and the procedure for [releases](./docs/release_procedure.md). You can also find a document briefly explaining the [project structure](./docs/project_structure.md).

Here you can also find the [dev activity log](./docs/dev_log.md), a document that thoroughly explains the project and each step taken to modify the original template. This document includes installation and running instructions, explanations on every component and a detailed walk through on each modification to the original node template.

## Getting Started

- 👉 Check the
[Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

- 🛠️ Depending on your operating system and Rust version, there might be additional
packages required to compile this template - please take note of the Rust compiler output.

Usually, it will be necessary to add the `wasm32v1-none` target, and the `rust-src` component, both of which can be installed, for example in Linux by executing the following commands:

```sh
$ rustup target add wasm32v1-none --toolchain stable-x86_64-unknown-linux-gnu
$ rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
```

- Fetch the code.

## Starting a local Chain

### Build your artifacts

You can build all of the artifacts like the node and the wallet:

```sh
cargo build --release -p griffin-partner-chains-node -p griffin-wallet
```

To run use:

```sh
target/release/griffin-partner-chains-node --dev --alice
```
With this command you can start a local development chain that will use predefined account Alice's keys, which are set in the runtime genesis as the authority keys.

### Interacting with the node

You can follow the instructions at [wallet](/wallet/README.md) to interact with the node using the Griffin wallet.

### Build and run with docker

:whale: Build the docker images that hold the node and wallet binaries compiled from the source code:

```sh
docker compose build 
```
Then run using:

```sh
docker compose up -d
```

This command initiates two containers that each run a validator node, using Alice and Bob predefined accounts. 
The docker utilizes a different `genesis` that sets these two accounts as authorities, because of this the resulting UTxOs change and so do the examples. In the [docker](/docker/) folder you can find the genesis that is used and the modified examples for testing. The examples are present in the docker containers.


### Interacting with the nodes through Docker

Similarly to the local chain, you can use Griffin wallet to interact with the node using `docker-exec`:

```sh
docker exec gpc-node-1-1 griffin-wallet -e http://localhost:9944 show-all-outputs 
```

Make sure that the endpoint you are connecting to matches the node's being run in the container.

## Devnet with Docker

To set up a partner-chain node, we need to have local instances of several services, namely: Ogmios, Db-sync, PostgreSQL and a Cardano node. For testing purposes we include a docker configuration that sets up this stack with a custom configuration for the Cardano node. This means that with the docker we can have a node of a local testnet available ready for setting up the partner-chain.

The stack can be found in [dev/local-environment](./dev/local-environment/). Here we have configuration files for the testnet and keys to setup partner chain nodes. We include the docker compose file that sets up the stack, but you can also run `bash setup.sh` to configure things like the PostgreSQL password, the ports for the services, and the memory and CPU limits for the services. 


