# Partnerchain reference implementation

This repository will contain a reference implementation of a Substrate partnerchain that can be used by developers in the ecosystem as blueprint / example to build their own specific chains. The project involves building a fully functional partnerchain within the context of a particular use-case.

## Table of Contents

- [Documentation](#documentation)

- [Getting Started](#getting-started)

- [Starting a chain](#starting-a-local-chain)

  - [Local chain](#build-your-artifacts)

  - [Start a chain with docker](#build-and-run-with-docker)

- [Use-case: Asteria game](#use-case-asteria-game)

- [Setting up a partner chain](#setting-up-a-partner-chain)

## Documentation

In the [docs](./docs/) folder you can find important information such as the [contribution guidelines](./docs/CONTRIBUTING.md) and the procedure for [releases](./docs/release_procedure.md). You can also find a document briefly explaining the [project structure](./docs/project_structure.md).

Here you can also find the [dev activity logs](./docs/dev_logs/). The [initial_customizations](./docs/dev_logs/initial_customizations.md) document thoroughly explains the project and each step taken to modify the original template. It includes installation and running instructions, explanations on every component and a detailed walk through on each modification to the original node template. The [partner_chain_integration](./docs/dev_logs/partner_chain_integration.md) document explains the modifications made to include the partner-chain features, modifications on the partner-chain features themselves, and a step-by-step guide on how to use the commands to set up the governance UTxOs on the Cardano side, using a local development testnet. The [use_case_implementation](./docs/dev_logs/use_case_implementation.md) document explains the modifications made to the node to add the use case specific features.

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

You can follow the instructions at [wallet](/wallet/README.md) to interact with the node using the Griffin wallet. Starting from `v0.0.4` the wallet is integrated into the node commands. This means you can use it directly with the node's executable:

```sh
./target/release/griffin-partner-chains-node wallet <wallet-subcommands>
```

> Note: this command only supports the wallet's subcommands. This means options like `--purge-db` are NOT supported. You can manually clean the wallet's database with `rm -r ~/.local/share/griffin-wallet/`. If you have the `griffin-wallet` executable, you can use options and commands like normal.

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

## Use-case: Asteria game

As the result of the discovery process, after surveying many developers, it was decided that the use case for the partnerchain reference implementation would be a game. We chose the game Asteria which consists of ships moving across a board to find the Asteria prize and collect a portion of it.

In this repository we include the [on-chain code](./game/onchain/), which comes with a [design document](./game/onchain/docs/design/design.md) that thoroughly explains the transactions involved in the game. In the game [src](./game/src/) you can find the implementation of the commands necessary to play the game and a [README](./game/README.md) that goes over the app design, in the context of the game's integration into the node, and [instructions](./game/README.md#game-usage) on how to run the commands.

## Setting up a Partner Chain

The project includes the `partner-chains-cli` from IOG's [Partner Chain SDK](https://github.com/input-output-hk/partner-chains). This `CLI` allows us to set up governance UTxOs on the Cardano side for the partner chain. An extensive explanation on the integration and usage of the CLI can be found at the previously mentioned [partner_chain_integration](./docs/dev_logs/partner_chain_integration.md) document.

## Devnet with Docker

To set up a partner-chain node, we need to have local instances of several services, namely: Ogmios, Db-sync, PostgreSQL and a Cardano node. For testing purposes we include a docker configuration that sets up this stack with a custom configuration for the Cardano node. This means that with the docker we can have a node of a local testnet available ready for setting up the partner-chain.

The stack can be found in [dev/local-environment](./dev/local-environment/). Here we have configuration files for the testnet and keys to setup partner chain nodes. We include the docker compose file that sets up the stack, but you can also run `bash setup.sh` to configure things like the PostgreSQL password, the ports for the services, and the memory and CPU limits for the services. 
