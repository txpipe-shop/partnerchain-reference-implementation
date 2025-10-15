# Partnerchain reference implementation

This repository will contain a reference implementation of a Substrate partnerchain that can be used by developers in the ecosystem as blueprint / example to build their own specific chains. The project involves building a fully functional partnerchain within the context of a particular use-case.

## Table of Contents

- [Documentation](#documentation)

- [Getting Started](#getting-started)

- [Starting a local chain](#starting-a-local-chain)

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

You can build all of the artifacts like the node and the wallet:

```sh
cargo build --release -p griffin-partner-chains-node -p griffin-wallet
```

To run use:

```sh
target/release/griffin-partner-chains-node --dev --alice
```
With this command you can start a local development chain that will use predefined account Alice's keys, which are set in the runtime genesis as the authority keys.