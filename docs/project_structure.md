# Project structure

There are three main components that any Polkadot-SDK project needs to function:
- the Node
- the Runtime
- the Pallets

## Runtime

The `runtime`, also referred to as a state transition function, refers to the core logic of the blockchain. That is, the logic around block validation and the execution of state changes.
The runtime is constructed using pallets, through which the behavior of the blockchain can be formed.

### State transition function

The runtime can be thought of as a state transition function because the behaviour of the blockchain is defined as on-chain modifications. A state transition in this case refers to these on-chain changes. In turn, for a Substrate-based blockchain, these are changes to the storage.
The state is, for example, the account balance of each user and where it is stored. 

## Pallets

A pallet is a modular component that encapsulates distinct functionalities or business logic. Pallets conform the building blocks for the construction of the blockhain's runtime.
There are many ready-made FRAME pallets provided by the polkadot-sdk and they can also be custom made using  provided by the polkadot-sdk as well.

## Node

The Node is the binary executable, whose primary purpose is to execute the runtime. It acts as an RPC server allowing interactions with the blockchain. Some of the important aspects defined here are the database, the peer-to-peer networking and even the consensus algorithm. 


Further reading:
- [Node vs Runtime](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/wasm_meta_protocol/index.html)
- [Blockchains as state machines](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/blockchain_state_machines/index.html)
- [Pallet macros](https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/index.html)