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

- Import Griffin types for `Transaction`, `Block`, `Executive` and `Output`.
- Define `SessionKeys` struct within `impl_opaque_keys` macro, with fields for `Aura` and `Grandpa` public keys.
- Remove `genesis_config_presets` mod definitions, since we are using our custom genesis.
- Declare `Runtime` struct without FRAME pallets.
- Implement custom `aura_authorities` and `grandpa_authorities` methods for Runtime.
- Remove all FRAME trait implementations for Runtime.

###### `impl_runtime_apis` macro

Runtime APIs are traits that are implemented in the runtime and provide both a runtime-side implementation and a client-side API for the node to interact with. To utilize Griffin we provide new implementations for the required traits.

- Core: use Griffin’s `Executive::execute_block` and `Executive::open_block` in `execute_block` and `initialize_block` methods implementations.
- Metadata: use trivial implementation.
- BlockBuilder: call Griffin’s `Executive::apply_extrinsic` and `Executive::close_block` in `apply_extrinsic` and `finalize_block` methods, and provide trivial implementations of `inherent_extrinsics` and `check_inherents` methods.
- TaggedTransactionQueue: use Griffin’s `Executive::validate_transaction`.
- SessionKeys: use the `generate` and `decode_into_raw_public_keys` methods of our defined `SessionKeys` type in `generate_session_keys` and `decode_session_keys` methods implementations.
- GenesisBuilder: use Griffin’s `GriffinGenesisConfigBuilder::build` and `get_genesis_config` functions to implement `build_state` and `get_preset` methods. Give trivial implementation of `preset_names`.
- Add `AuraApi` and `GrandpaApi` trait implementations.
- Remove OffchainWorkerApi, AccountNonceApi and TransactionPaymentApi trait implementations.

### Changes made to Griffin

As mentioned, the version of Griffin that we use for this project has some modifications compared to the original. Most of these changes are dependency upgrades, but below we'll go over other more interesting modifications:
- [Authorities set function]: we re-implemented the authorities setting function to utilize the EUTxO model. The new function reads the authorities list from a UTxO that is set in the Genesis. A more detailed explanation on how it works and how to use it can be found in the respective readme.
- [Griffin-RPC]: We extended the native node RPC with some queries to obtain UTxOs information through an output reference, an address, or an asset class. More over, we also added a method to submit a transaction in CBOR format. More information and usage examples can be found in the Griffin RPC [readme].
- [Wallet]: The wallet was also improved on through the addition of new functionalities like the queries by address and asset. The `build-tx` command was also modified to take as input a whole json file, instead of many arguments for each component of the transaction.

## Guide to Griffin

### Types

### Wallet

Griffin provides a CLI wallet to interact with the node. This wallet has helpful commands:
`show-all-outputs`: Displays every UTxO in the chain with brief details about the owner, the coins and value.
`show-outputs-at`: Displays UTxOs owned by the provided address.
`show-outputs-with-asset`: Displays UTxOs that contain a certain token in its value.
`insert-key`: Inserts a key into the wallets keystore.
`generate-key`: Creates a new key and inserts it into the keystore. Also displays the details.
`show-balance`: summarizes Value amounts for each address.
`build-tx`: Transaction builder command, which takes as input a complete Griffin transaction in json. This transaction must be balanced manually. 

More information can be found in the wallet [readme] and also there are some usage examples in the [examples folder].

## Troubleshooting

These are some common errors that can happen when developing on Substrate:

### STD related issues

Errors like:

- `Double lang item in crate <crate> (which `std`/ `serde` depends on):...` 
- `Attempted to define built-in macro more than once`

happen commonly when using std crates in a non-std environment, like Substrate's runtime. Std crates can't be used because we compile to WASM. If you run into an error like this and the crate you are using is no-std, make sure you are setting them up correctly. For example, make sure that the dependency is imported with `default-features = false` or that the std feature is set correctly in the respective `Cargo.toml`. If you are writing a new module, make sure that it is premised by ´#![cfg_attr(not(feature = "std"), no_std)]´.

### `Alloc` feature

When trying to use `alloc` features like `vec`, you might run into the trouble that the compiler can't find the `alloc` crate. This feature can be imported from various dependencies like `serde` and `serde_json`. To use it make sure to add `extern crate alloc;` at the top of your file.

