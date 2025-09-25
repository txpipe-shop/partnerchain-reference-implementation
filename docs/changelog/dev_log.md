# Dev activity log

This is a detailed log of the development activities that were required for the Substrate client customization. We’d like this log to serve as guidance for new developers wishing to understand the process with the goal of implementing their own versions.

## :checkered_flag: Starting point: Substrate Template

### :down_arrow: Download the [minimal template](https://github.com/paritytech/polkadot-sdk-minimal-template).

This version of the node includes the most basic featurs of a Substrate node.

### :computer: Rust setup & dependencies:

#### Rust installation

Check the Rust [installation instructions](https://www.rust-lang.org/tools/install) for your system.

#### Additional packages

Depending on your system and Rust version, there might be additional packages required to compile this template - please take note of the Rust compiler output. Usually, it will be necessary to add the `wasm32-unknown-unknown` target, and the `rust-src` component, both of which can be installed, for example in Linux by executing the following commands:

```bash
$ rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu
$ rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
```

## :book: Understand Substrate.

### Basic components

#### Runtime

The runtime represents the [state transition function](https://docs.polkadot.com/polkadot-protocol/glossary/#state-transition-function-stf) for a blockchain. In Polkadot SDK, the runtime is stored as a [Wasm](https://docs.polkadot.com/polkadot-protocol/glossary/#webassembly-wasm) binary in the chain state. The Runtime is stored under a unique state key and can be modified during the execution of the state transition function.

#### Node (Client)

The node, also known as the client, is the core component responsible for executing the Wasm runtime and orchestrating various essential blockchain components. It ensures the correct execution of the state transition function and manages multiple critical subsystems, including:
- *Wasm execution:* Runs the blockchain runtime, which defines the state transition rules.
- *Database management:* Stores blockchain data.
- *Networking:* Facilitates peer-to-peer communication, block propagation, and transaction gossiping.
- *Transaction pool (Mempool):* Manages pending transactions before they are included in a block.
- *Consensus mechanism:* Ensures agreement on the blockchain state across nodes.
- *RPC services:* Provides external interfaces for applications and users to interact with the node.

## :eagle: Include Griffin

As part of the Substrate customizations that can be done, we can modify the ledger model and the storage structure. This is where [Griffin](github.com/txpipe/griffin) comes in. Griffin is a Substrate-based clone of a Cardano node. It incorporates a simplified eUTxO ledger and hosts a virtual machine capable of executing Plutus scripts for app-logic. This setup provides the essential primitives and logic required for our appchain nodes.

### Griffin vs. Substrate

A common Substrate node uses an account model ledger and its runtime is built with FRAME pallets, which are runtime "building blocks" or modules. In contrast, Griffin is designed for the UTxO model, making it incompatible with FRAME, as pallets inherently assume an account-oriented underlying model.

This imposes a restriction on the design and implementation, opposed to usual Substrate app-chains. For developers coming from the Cardano ecosystem though, Griffin provides a familiar environment. We can think about decentralized applications in terms of validators and UTxOs, with the advantage of having a completely modifiable starting point as well.

The original Griffin is a bit outdated, so to use it we updated some dependencies and made some minor compatibility changes, which will be detailed below. If you wish to use Griffin, we encourage the use of the ad-hoc version provided in this repository at [griffin-core].

### Changes made to the template to add Griffin

To add Griffin to the template, these are the changes that have to be made:

#### Runtime

A new [genesis](../../runtime/src/) file that includes the information for the initial set of UTxOs and a `get_genesis_config` function to build the genesis in the runtime.

In the [runtime library](../../runtime/src/lib.rs):

##### Type definitions

- Import Griffin types for Transaction, Block, Executive and Output.


## Griffin usage guide

## Troubleshooting

These are some common errors that can happen when developing on Substrate:

### STD related issues

Errors like:

- `Double lang item in crate <crate> (which `std`/ `serde` depends on):...` 
- `Attempted to define built-in macro more than once`

happen commonly when using std crates in a non-std environment, like Substrate's runtime. Std crates can't be used because we compile to WASM. If you run into an error like this and the crate you are using is no-std, make sure you are setting them up correctly. For example, make sure that the dependency is imported with `default-features = false` or that the std feature is set correctly in the respective `Cargo.toml`. If you are writing a new module, make sure that it is premised by ´#![cfg_attr(not(feature = "std"), no_std)]´.

### `Alloc` feature

When trying to use `alloc` features like `vec`, you might run into the trouble that the compiler can't find the `alloc` crate. This feature can be imported from various dependencies like `serde` and `serde_json`. To use it make sure to add `extern crate alloc;` at the top of your file.

