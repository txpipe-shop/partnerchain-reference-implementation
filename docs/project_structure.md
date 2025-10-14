# Project structure

There are the main components of this Polkadot-sdk project:
- the Node
- the Runtime
- Griffin

## Runtime

The `runtime`, also referred to as a state transition function, refers to the core logic of the blockchain. That is, the logic around block validation and the execution of state changes.
In this project the key structures of the runtime are defined using [Griffin](https://docs.txpipe.io/griffin) which provides the following characteristics:

- UTxO-based ledger: the ledger resembles the Cardano ledger as much as possible, with the exception of any staking, delegation or governance primitives.
- Extended UTxO primitives: it replicates the programability primitives around UTxOs (datums, redeemers, scripts, etc).
- Plutus VM: it integrates a virtual machine capable of executing Plutus scripts that can be created using existing Plutus tooling and languages, such as Aiken.

### State transition function

The runtime can be thought of as a state transition function because the behaviour of the blockchain is defined as on-chain modifications. A state transition in this case refers to these on-chain changes.

## Node

The Node is the binary executable, whose primary purpose is to execute the runtime. It acts as an RPC server allowing interactions with the blockchain. Some of the important aspects defined here are the database, the peer-to-peer networking and even the consensus algorithm. 

## Griffin

Griffin core is the source of the definitions and implementations necessary for the runtime. The source code is extensive but the most important modules are the following:
- Types: type definitions from Block, Transaction, WitnessSet, and more.
- RPC: implementations for the RPC queries. 

Further reading:
- [Node vs Runtime](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/wasm_meta_protocol/index.html)
- [Blockchains as state machines](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/blockchain_state_machines/index.html)