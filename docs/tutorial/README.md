# Development Tutorial

In this Tutorial, we will walk you through all the steps necessary to customize a generic Substrate
node into a Partnerchain node with use-case specific features.
 
The material presented here is a streamlined version of the info detailed at the [Dev Activity
Logs](dev_logs/) ([node customization](dev_logs/initial_customizations.md), [Partner Chains SDK
integration](dev-logs/partner_chain_integration.md)).

## Summary

This reference implementation takes the very basic [Substrate's minimal
template](https://github.com/paritytech/polkadot-sdk-minimal-template) and shows how to: 

- add a custom ledger (eUTxO using Griffin);
- set up consensus (Aura) and finality (GRANDPA) algorithms;
- integrate the Partner Chains (PC) SDK; and
- set up an application (Asteria).

We hope that the guide presented here helps you to set your particular use-case.

### Index

- [Node customization](#node-customization) involves:
  - [Installing the ledger](#installing-the-ledger) explains the process of adding the Griffin
    ledger. This requires extensive editing of the runtime, which is detailed in [Runtime
    sources](#runtime-sources). The node client requires fewer modifications, which are detailed in
    [Node Sources](#node-sources).

  - [Partnerchain integration](#partnerchain-integration).

- [Operating instructions](#operating-instructions).

- [Troubleshooting](#troubleshooting) addresses some common pitfalls while editing and building
  Substrate nodes.

## Node customization

### Requisites

For concreteness' sake, we run this tutorial on Linux. The following required elements are specified
with the corresponding tested versions.

1. Sources from the [PSDK minimal
   template](https://github.com/paritytech/polkadot-sdk-minimal-template) repo, on its
   [update](https://github.com/paritytech/polkadot-sdk-minimal-template/commit/c6f71c73f61bc8bc78574037fed712f8046b57c7)
   to PSDK `stable2503` (the latest one at the time of this writing).
2. Rust `1.90.0-x86_64-unknown-linux-gnu` and the additional dependencies needed for your system
   (check Polkadot's official [installation
   instructions](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/) for that).
3. Rust's target `wasm32v1-none` and the
   `rust-src` component, which can be installed by executing the following commands:

   ```bash
   $ rustup target add wasm32v1-none --toolchain stable-x86_64-unknown-linux-gnu
   $ rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
   ```
4. Sources from IOG's [Partner Chains (PC)
   SDK](https://github.com/input-output-hk/partner-chains/tree/master) at [release
   `v1.8.0`](https://github.com/input-output-hk/partner-chains/releases/tag/v1.8.0).
5. The modified [Griffin](https://github.com/txpipe/griffin/releases/tag/catalyst-ms5/) sources at
   this repo (comprising the directories [griffin-core](../griffin-core),
   [griffin-rpc](../griffin-rpc), [wallet](../wallet), and [demo](../demo)).

For a quick summary of the Substrate Node components, check [this Section of the
dev-logs](dev_logs/initial_customizations.md#book-understand-substrate).

[Here](dev_logs/initial_customizations.md#griffin-vs-substrate) you can find an introduction to
Griffin, the [changes and updates](dev_logs/initial_customizations.md#changes-made-to-griffin) to
it, and a [brief reference](dev_logs/initial_customizations.md#guide-to-griffin) to its types and
wallet commands.

### Installing the ledger

#### Dependencies

##### Workspace dependencies

First, copy the directories [griffin-core](../griffin-core), [griffin-rpc](../griffin-rpc),
[wallet](../wallet), and [demo](../demo) to the root of the Minimal Template. We add them to
workspace. Since we are setting up an eUTxO model for the node's ledger (in contrast to Substrate's
default account model), FRAME pallets are not to be used, and we also remove them:

``` diff
--- a/Cargo.toml
+++ b/Cargo.toml
 edition = "2021"
 
 [workspace]
-default-members = ["pallets/template", "runtime"]
+default-members = ["runtime"]
 members = [
     "node",
-    "pallets/template",
     "runtime",
+    "demo/authorities",
+    "griffin-core",
+    "griffin-rpc",
+    "wallet",
 ]
 resolver = "2"
 
 [workspace.dependencies]
-minimal-template-runtime = { path = "./runtime", default-features = false }
-pallet-minimal-template = { path = "./pallets/template", default-features = false }
+griffin-core = { default-features = false, path = "griffin-core" }
+griffin-partner-chains-runtime = { path = "./runtime", default-features = false }
+griffin-rpc = { default-features = false, path = "griffin-rpc" }
 clap = { version = "4.5.13" }
 docify = { version = "0.2.9" }
 futures = { version = "0.3.31" }
```

We also add Griffin dependencies:

``` diff
 scale-info = { version = "2.11.6", default-features = false }
 serde_json = { version = "1.0.132", default-features = false }
 
+# griffin deps
+sc-chain-spec = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2", default-features = false }
+
+# demo implementations
+demo-authorities = { path = "demo/authorities", default-features = false }
+
 [profile.release]
 opt-level = 3
 panic = "unwind"

```

> [!NOTE]
>
> In this project we decided to steer away from using `polkadot-sdk` as one big dependency. Instead,
> we pick and choose what we need from the Polkadot-SDK and set each as their own dependency, using
> the Git tag for release `polkadot-stable2506-2`.

Click the block below to see the rather long diff:

<details>
  <summary>

``` diff
 docify = { version = "0.2.9" }
 futures = { version = "0.3.31" }
 futures-timer = { version = "3.0.2" }
 jsonrpsee = { version = "0.24.3" }
-polkadot-sdk = { version = "2503.0.1", default-features = false }
+frame-support = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+frame-system = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+polkadot-sdk-frame = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-basic-authorship = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
# ... snip ...
```
  </summary>

``` diff
# ... continued ...
+sc-cli = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-client-api = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-consensus = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-consensus-manual-seal = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-executor = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-keystore = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-network = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-offchain = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-service = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-telemetry = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-transaction-pool = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sc-transaction-pool-api = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-api = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-application-crypto = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-block-builder = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-blockchain = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-consensus-aura = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-consensus-grandpa = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-core = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-debug-derive = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2", default-features = false }
+sp-genesis-builder = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-inherents = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-io = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-keystore = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-runtime = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-session = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-storage = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-timestamp = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-transaction-pool = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-version = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+substrate-build-script-utils = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+substrate-frame-rpc-system = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+substrate-wasm-builder = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+derive-new = { version = "0.7.0" }
+hex = { version = "0.4.3", features = ["alloc"], default-features = false }
+hex-literal = "1.0.0"
+log = { version = "0.4", default-features = false }
+parity-scale-codec = { package = "parity-scale-codec", version = "3.6.12", default-features = false, features = [
+	"derive",
+	"max-encoded-len",
+] }
+serde = { version = "1.0.209", default-features = false, features = [
+	"derive",
+	"alloc",
+] }
+thiserror = { version = "2.0", default-features = false }
 codec = { version = "3.7.4", default-features = false, package = "parity-scale-codec" }
 scale-info = { version = "2.11.6", default-features = false }
 serde_json = { version = "1.0.132", default-features = false }
```
</details>

##### Package dependencies

This processes has to be repeated for all packages in the workspace. For the node,

``` diff
--- a/node/Cargo.toml
+++ b/node/Cargo.toml
@@ -20,14 +20,33 @@ futures = { features = ["thread-pool"], workspace = true }
 futures-timer = { workspace = true }
 jsonrpsee = { features = ["server"], workspace = true }
 minimal-template-runtime.workspace = true
-polkadot-sdk = { workspace = true, features = ["experimental", "node"] }
+griffin-core = { workspace = true }
+griffin-rpc = { workspace = true }
+sc-basic-authorship = { workspace = true }
+sc-cli = { workspace = true }
+sc-client-api = { workspace = true }
+sc-consensus = { workspace = true }
+sc-consensus-manual-seal = { workspace = true }
+sc-executor = { workspace = true }
+sc-network = { workspace = true }
+sc-service = { workspace = true }
+sc-telemetry = { workspace = true }
+sc-transaction-pool = { workspace = true }
+sc-transaction-pool-api = { workspace = true }
+serde_json = { workspace = true }
+sp-api = { workspace = true }
+sp-block-builder = { workspace = true }
+sp-blockchain = { workspace = true }
+sp-genesis-builder = { workspace = true }
+sp-io = { workspace = true }
+sp-timestamp = { workspace = true }
+sp-runtime = { workspace = true }
 
 [build-dependencies]
-polkadot-sdk = { workspace = true, features = ["substrate-build-script-utils"] }
+substrate-build-script-utils = { workspace = true, default-features = true }
 
 [features]
 default = ["std"]
 std = [
 	"minimal-template-runtime/std",
-	"polkadot-sdk/std",
 ]
```
and for the runtime:

``` diff
--- a/runtime/Cargo.toml
+++ b/runtime/Cargo.toml
@@ -10,21 +10,41 @@ edition.workspace = true
 publish = false
 
 [dependencies]
-codec = { workspace = true }
-pallet-minimal-template.workspace = true
-polkadot-sdk = { workspace = true, features = ["pallet-balances", "pallet-sudo", "pallet-timestamp", "pallet-transaction-payment", "pallet-transaction-payment-rpc-runtime-api", "runtime"] }
+demo-authorities = { workspace = true }
+griffin-core = { workspace = true }
+parity-scale-codec = { features = ["derive"], workspace = true }
+polkadot-sdk-frame = { features = ["runtime"], workspace = true }
 scale-info = { workspace = true }
 serde_json = { workspace = true, default-features = false, features = ["alloc"] }
+sp-api = { workspace = true }
+sp-application-crypto = { workspace = true }
+sp-block-builder = { workspace = true }
+sp-consensus-aura = { workspace = true }
+sp-consensus-grandpa = { workspace = true }
+sp-core = { workspace = true }
+sp-genesis-builder = { workspace = true }
+sp-inherents = { workspace = true }
+sp-runtime = { workspace = true }
+sp-session = { workspace = true }
+sp-transaction-pool = { workspace = true }
+sp-version = { workspace = true }
+
+serde = { workspace = true }
 
 [build-dependencies]
-polkadot-sdk = { optional = true, workspace = true, features = ["substrate-wasm-builder"] }
+substrate-wasm-builder = { optional = true, workspace = true, default-features = true }
 
 [features]
 default = ["std"]
 std = [
-	"codec/std",
-	"pallet-minimal-template/std",
-	"polkadot-sdk/std",
+	"griffin-core/std",
+	"polkadot-sdk-frame/std",
 	"scale-info/std",
 	"serde_json/std",
+	"sp-block-builder/std",
+	"sp-consensus-aura/std",
+	"sp-consensus-grandpa/std",
+	"sp-session/std",
+	"sp-transaction-pool/std",
+	"substrate-wasm-builder",
 ]
```

#### Runtime sources

Setting up UTxO logic on top of a Polkadot/Substrate node requires extensive changes to the
runtime. We will highlight below some of the steps involved.

Recall that we took the approach of replacing `polkadot-sdk` by several smaller packages. This
implies that some imports have to replaced by the corresponding replacements. For instance, at
`build.rs` we have the following replacement:

``` diff
 fn main() {
-	#[cfg(feature = "std")]
-	{
-		polkadot_sdk::substrate_wasm_builder::WasmBuilder::build_using_defaults();
-	}
+    #[cfg(feature = "std")]
+    {
+        substrate_wasm_builder::WasmBuilder::build_using_defaults();
+    }
 }
```

Similar edits have to be carried out throughout the sources, which we will omit henceforth.

##### Genesis

Add a new [genesis](../runtime/src/genesis.rs) file that includes the information for the initial set of UTxOs and a `get_genesis_config` function to build the genesis in the runtime.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/genesis.rs#L57-L67

##### Runtime library

Edit the [runtime library](../../runtime/src/lib.rs) as indicated:

- Import Griffin types for  `Address`, `AssetName`, `Datum`, `Input` and `PolicyId`. These types will be used to implement the runtime apis necessary for Griffin RPC. 
- Import `TransparentUTxOSet` from Griffin.
- Import `MILLI_SECS_PER_SLOT` from Griffin, which will be used to define the slot duration of the chain.
- Import `GenesisConfig` from Griffin's config builder.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L14-L18

- Import Authorities from `demo`, which holds custom `aura_authorities` and `grandpa_authorities` implementations.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L34

- Define `SessionKeys` struct within `impl_opaque_keys` macro, with fields for `Aura` and `Grandpa` public keys.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L44-L75

- Remove `genesis_config_presets` mod definitions, since we are using our custom genesis.
- Define `Transaction`, `Block`, `Executive` and `Output` using the imported types from Griffin.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L99-L102

- Declare `Runtime` struct without FRAME pallets.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L105-L106

- Remove all FRAME trait implementations for Runtime.

##### `impl_runtime_apis` macro

Runtime APIs are traits that are implemented in the runtime and provide both a runtime-side implementation and a client-side API for the node to interact with. To utilize Griffin we provide new implementations for the required traits.

- Core: use Griffin’s `Executive::execute_block` and `Executive::open_block` in `execute_block` and `initialize_block` methods implementations.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L124-L136
- Metadata: use trivial implementation.

- BlockBuilder: call Griffin’s `Executive::apply_extrinsic` and `Executive::close_block` in `apply_extrinsic` and `finalize_block` methods, and provide trivial implementations of `inherent_extrinsics` and `check_inherents` methods.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L152-L163

- TaggedTransactionQueue: use Griffin’s `Executive::validate_transaction`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L173-L181

- SessionKeys: use the `generate` and `decode_into_raw_public_keys` methods of our defined `SessionKeys` type in `generate_session_keys` and `decode_session_keys` methods implementations.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L183-L193

- GenesisBuilder: use Griffin’s `GriffinGenesisConfigBuilder::build` and `get_genesis_config` functions to implement `build_state` and `get_preset` methods. Give trivial implementation of `preset_names`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L232-L249

- Include `sp_consensus_aura::AuraApi<Block, AuraId> `. Use custom `aura_authorities` implementation for `authorities` method. Use `SlotDuration::from_millis` from `sp_consensus_aura` with previously imported MILLI_SECS_PER_SLOT to define `slot_duration`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L195-L203

- Include `sp_consensus_grandpa::GrandpaApi<Block>`. Use custom `grandpa_authorities` implementation for the homonymous function from the api. Give a trivial implementation for `current_set_id`, `submit_report_equivocation_unsigned_extrinsic` and `generate_key_ownership_proof`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L205-L222

- Include `griffin_core::utxo_set::TransparentUtxoSetApi<Block>`. Use `peek_utxo`, `peek_utxo_from_address` and `peek_utxo_with_asset` from `TransparentUtxoSet` to implement `peek_utxo`, `peek_utxo_by_address` and `peek_utxo_with_asset` from the api, respectively.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/runtime/src/lib.rs#L109-L121

- Remove `OffchainWorkerApi`, `AccountNonceApi` and `TransactionPaymentApi` trait implementations.

<details>
  <summary>

**Complete `runtime/src/lib.rs` diff** (click to expand)
  </summary>

``` diff
--- a/runtime/src/lib.rs
+++ b/runtime/src/lib.rs
@@ -1,21 +1,4 @@
-// This file is part of Substrate.
-
-// Copyright (C) Parity Technologies (UK) Ltd.
-// SPDX-License-Identifier: Apache-2.0
-
-// Licensed under the Apache License, Version 2.0 (the "License");
-// you may not use this file except in compliance with the License.
-// You may obtain a copy of the License at
-//
-// 	http://www.apache.org/licenses/LICENSE-2.0
-//
-// Unless required by applicable law or agreed to in writing, software
-// distributed under the License is distributed on an "AS IS" BASIS,
-// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-// See the License for the specific language governing permissions and
-// limitations under the License.
-
-//! A minimal runtime that includes the template [`pallet`](`pallet_minimal_template`).
+//! The runtime contains the core logic of the ledger run by the Griffin node.
 
 #![cfg_attr(not(feature = "std"), no_std)]
 
@@ -25,325 +8,243 @@ include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
 
 extern crate alloc;
 
-use alloc::vec::Vec;
-use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
-use polkadot_sdk::{
-	polkadot_sdk_frame::{
-		self as frame,
-		deps::sp_genesis_builder,
-		runtime::{apis, prelude::*},
-	},
-	*,
+pub mod genesis;
+
+use alloc::{string::ToString, vec, vec::Vec};
+use griffin_core::genesis::config_builder::GenesisConfig;
+use griffin_core::types::{Address, AssetName, Input, PolicyId};
+use griffin_core::utxo_set::TransparentUtxoSet;
+use griffin_core::MILLI_SECS_PER_SLOT;
+pub use opaque::SessionKeys;
+
+use parity_scale_codec::{Decode, Encode};
+use polkadot_sdk_frame::runtime::apis;
+use scale_info::TypeInfo;
+use sp_api::impl_runtime_apis;
+use sp_consensus_aura::sr25519::AuthorityId as AuraId;
+use sp_core::OpaqueMetadata;
+use sp_inherents::InherentData;
+use sp_runtime::{
+    impl_opaque_keys,
+    traits::Block as BlockT,
+    transaction_validity::{TransactionSource, TransactionValidity},
+    ApplyExtrinsicResult, BoundToRuntimeAppPublic,
 };
 
-/// Provides getters for genesis configuration presets.
-pub mod genesis_config_presets {
-	use super::*;
-	use crate::{
-		interface::{Balance, MinimumBalance},
-		sp_keyring::Sr25519Keyring,
-		BalancesConfig, RuntimeGenesisConfig, SudoConfig,
-	};
-
-	use alloc::{vec, vec::Vec};
-	use serde_json::Value;
-
-	/// Returns a development genesis config preset.
-	pub fn development_config_genesis() -> Value {
-		let endowment = <MinimumBalance as Get<Balance>>::get().max(1) * 1000;
-		frame_support::build_struct_json_patch!(RuntimeGenesisConfig {
-			balances: BalancesConfig {
-				balances: Sr25519Keyring::iter()
-					.map(|a| (a.to_account_id(), endowment))
-					.collect::<Vec<_>>(),
-			},
-			sudo: SudoConfig { key: Some(Sr25519Keyring::Alice.to_account_id()) },
-		})
-	}
-
-	/// Get the set of the available genesis config presets.
-	pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
-		let patch = match id.as_ref() {
-			sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
-			_ => return None,
-		};
-		Some(
-			serde_json::to_string(&patch)
-				.expect("serialization to json is expected to work. qed.")
-				.into_bytes(),
-		)
-	}
-
-	/// List of supported presets.
-	pub fn preset_names() -> Vec<PresetId> {
-		vec![PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET)]
-	}
+use demo_authorities as Authorities;
+
+#[cfg(feature = "std")]
+use sp_version::NativeVersion;
+use sp_version::{runtime_version, RuntimeVersion};
+
+/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
+/// the specifics of the runtime. They can then be made to be agnostic over specific formats
+/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
+/// to even the core data structures.
+pub mod opaque {
+    use super::*;
+    use sp_core::{ed25519, sr25519};
+
+    // This part is necessary for generating session keys in the runtime
+    impl_opaque_keys! {
+        pub struct SessionKeys {
+            pub aura: AuraAppPublic,
+            pub grandpa: GrandpaAppPublic,
+        }
+    }
+    impl From<(sr25519::Public, ed25519::Public)> for SessionKeys {
+        fn from((aura, grandpa): (sr25519::Public, ed25519::Public)) -> Self {
+            Self {
+                aura: aura.into(),
+                grandpa: grandpa.into(),
+            }
+        }
+    }
+    // Typically these are not implemented manually, but rather for the pallet associated with the
+    // keys. Here we are not using the pallets, and these implementations are trivial, so we just
+    // re-write them.
+    pub struct AuraAppPublic;
+    impl BoundToRuntimeAppPublic for AuraAppPublic {
+        type Public = AuraId;
+    }
+
+    pub struct GrandpaAppPublic;
+    impl BoundToRuntimeAppPublic for GrandpaAppPublic {
+        type Public = sp_consensus_grandpa::AuthorityId;
+    }
 }
 
 /// The runtime version.
 #[runtime_version]
 pub const VERSION: RuntimeVersion = RuntimeVersion {
-	spec_name: alloc::borrow::Cow::Borrowed("minimal-template-runtime"),
-	impl_name: alloc::borrow::Cow::Borrowed("minimal-template-runtime"),
-	authoring_version: 1,
-	spec_version: 0,
-	impl_version: 1,
-	apis: RUNTIME_API_VERSIONS,
-	transaction_version: 1,
-	system_version: 1,
+    spec_name: alloc::borrow::Cow::Borrowed("griffin-solochain-runtime"),
+    impl_name: alloc::borrow::Cow::Borrowed("griffin-solochain-runtime"),
+    authoring_version: 1,
+    spec_version: 0,
+    impl_version: 1,
+    apis: RUNTIME_API_VERSIONS,
+    transaction_version: 1,
+    system_version: 1,
 };
 
 /// The version information used to identify this runtime when compiled natively.
 #[cfg(feature = "std")]
 pub fn native_version() -> NativeVersion {
-	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
-}
-
-/// The transaction extensions that are added to the runtime.
-type TxExtension = (
-	// Checks that the sender is not the zero address.
-	frame_system::CheckNonZeroSender<Runtime>,
-	// Checks that the runtime version is correct.
-	frame_system::CheckSpecVersion<Runtime>,
-	// Checks that the transaction version is correct.
-	frame_system::CheckTxVersion<Runtime>,
-	// Checks that the genesis hash is correct.
-	frame_system::CheckGenesis<Runtime>,
-	// Checks that the era is valid.
-	frame_system::CheckEra<Runtime>,
-	// Checks that the nonce is valid.
-	frame_system::CheckNonce<Runtime>,
-	// Checks that the weight is valid.
-	frame_system::CheckWeight<Runtime>,
-	// Ensures that the sender has enough funds to pay for the transaction
-	// and deducts the fee from the sender's account.
-	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
-	// Reclaim the unused weight from the block using post dispatch information.
-	// It must be last in the pipeline in order to catch the refund in previous transaction
-	// extensions
-	frame_system::WeightReclaim<Runtime>,
-);
-
-// Composes the runtime by adding all the used pallets and deriving necessary types.
-#[frame_construct_runtime]
-mod runtime {
-	/// The main runtime type.
-	#[runtime::runtime]
-	#[runtime::derive(
-		RuntimeCall,
-		RuntimeEvent,
-		RuntimeError,
-		RuntimeOrigin,
-		RuntimeFreezeReason,
-		RuntimeHoldReason,
-		RuntimeSlashReason,
-		RuntimeLockId,
-		RuntimeTask,
-		RuntimeViewFunction
-	)]
-	pub struct Runtime;
-
-	/// Mandatory system pallet that should always be included in a FRAME runtime.
-	#[runtime::pallet_index(0)]
-	pub type System = frame_system::Pallet<Runtime>;
-
-	/// Provides a way for consensus systems to set and check the onchain time.
-	#[runtime::pallet_index(1)]
-	pub type Timestamp = pallet_timestamp::Pallet<Runtime>;
-
-	/// Provides the ability to keep track of balances.
-	#[runtime::pallet_index(2)]
-	pub type Balances = pallet_balances::Pallet<Runtime>;
-
-	/// Provides a way to execute privileged functions.
-	#[runtime::pallet_index(3)]
-	pub type Sudo = pallet_sudo::Pallet<Runtime>;
-
-	/// Provides the ability to charge for extrinsic execution.
-	#[runtime::pallet_index(4)]
-	pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;
-
-	/// A minimal pallet template.
-	#[runtime::pallet_index(5)]
-	pub type Template = pallet_minimal_template::Pallet<Runtime>;
-}
-
-parameter_types! {
-	pub const Version: RuntimeVersion = VERSION;
+    NativeVersion {
+        runtime_version: VERSION,
+        can_author_with: Default::default(),
+    }
 }
 
-/// Implements the types required for the system pallet.
-#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
-impl frame_system::Config for Runtime {
-	type Block = Block;
-	type Version = Version;
-	// Use the account data from the balances pallet
-	type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
-}
-
-// Implements the types required for the balances pallet.
-#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
-impl pallet_balances::Config for Runtime {
-	type AccountStore = System;
-}
-
-// Implements the types required for the sudo pallet.
-#[derive_impl(pallet_sudo::config_preludes::TestDefaultConfig)]
-impl pallet_sudo::Config for Runtime {}
-
-// Implements the types required for the sudo pallet.
-#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
-impl pallet_timestamp::Config for Runtime {}
-
-// Implements the types required for the transaction payment pallet.
-#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig)]
-impl pallet_transaction_payment::Config for Runtime {
-	type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
-	// Setting fee as independent of the weight of the extrinsic for demo purposes
-	type WeightToFee = NoFee<<Self as pallet_balances::Config>::Balance>;
-	// Setting fee as fixed for any length of the call data for demo purposes
-	type LengthToFee = FixedFee<1, <Self as pallet_balances::Config>::Balance>;
-}
+pub type Transaction = griffin_core::types::Transaction;
+pub type Block = griffin_core::types::Block;
+pub type Executive = griffin_core::Executive;
+pub type Output = griffin_core::types::Output;
 
-// Implements the types required for the template pallet.
-impl pallet_minimal_template::Config for Runtime {}
-
-type Block = frame::runtime::types_common::BlockOf<Runtime, TxExtension>;
-type Header = HeaderFor<Runtime>;
-
-type RuntimeExecutive =
-	Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;
+/// The main struct in this module.
+#[derive(Encode, Decode, PartialEq, Eq, Clone, TypeInfo)]
+pub struct Runtime;
 
 impl_runtime_apis! {
-	impl apis::Core<Block> for Runtime {
-		fn version() -> RuntimeVersion {
-			VERSION
-		}
-
-		fn execute_block(block: Block) {
-			RuntimeExecutive::execute_block(block)
-		}
-
-		fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
-			RuntimeExecutive::initialize_block(header)
-		}
-	}
-	impl apis::Metadata<Block> for Runtime {
-		fn metadata() -> OpaqueMetadata {
-			OpaqueMetadata::new(Runtime::metadata().into())
-		}
-
-		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
-			Runtime::metadata_at_version(version)
-		}
-
-		fn metadata_versions() -> Vec<u32> {
-			Runtime::metadata_versions()
-		}
-	}
-
-	impl apis::BlockBuilder<Block> for Runtime {
-		fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
-			RuntimeExecutive::apply_extrinsic(extrinsic)
-		}
-
-		fn finalize_block() -> HeaderFor<Runtime> {
-			RuntimeExecutive::finalize_block()
-		}
-
-		fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
-			data.create_extrinsics()
-		}
-
-		fn check_inherents(
-			block: Block,
-			data: InherentData,
-		) -> CheckInherentsResult {
-			data.check_extrinsics(&block)
-		}
-	}
-
-	impl apis::TaggedTransactionQueue<Block> for Runtime {
-		fn validate_transaction(
-			source: TransactionSource,
-			tx: ExtrinsicFor<Runtime>,
-			block_hash: <Runtime as frame_system::Config>::Hash,
-		) -> TransactionValidity {
-			RuntimeExecutive::validate_transaction(source, tx, block_hash)
-		}
-	}
-
-	impl apis::OffchainWorkerApi<Block> for Runtime {
-		fn offchain_worker(header: &HeaderFor<Runtime>) {
-			RuntimeExecutive::offchain_worker(header)
-		}
-	}
-
-	impl apis::SessionKeys<Block> for Runtime {
-		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
-			Default::default()
-		}
-
-		fn decode_session_keys(
-			_encoded: Vec<u8>,
-		) -> Option<Vec<(Vec<u8>, apis::KeyTypeId)>> {
-			Default::default()
-		}
-	}
-
-	impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
-		fn account_nonce(account: interface::AccountId) -> interface::Nonce {
-			System::account_nonce(account)
-		}
-	}
-
-	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
-		Block,
-		interface::Balance,
-	> for Runtime {
-		fn query_info(uxt: ExtrinsicFor<Runtime>, len: u32) -> RuntimeDispatchInfo<interface::Balance> {
-			TransactionPayment::query_info(uxt, len)
-		}
-		fn query_fee_details(uxt: ExtrinsicFor<Runtime>, len: u32) -> FeeDetails<interface::Balance> {
-			TransactionPayment::query_fee_details(uxt, len)
-		}
-		fn query_weight_to_fee(weight: Weight) -> interface::Balance {
-			TransactionPayment::weight_to_fee(weight)
-		}
-		fn query_length_to_fee(length: u32) -> interface::Balance {
-			TransactionPayment::length_to_fee(length)
-		}
-	}
-
-	impl apis::GenesisBuilder<Block> for Runtime {
-		fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
-			build_state::<RuntimeGenesisConfig>(config)
-		}
-
-		fn get_preset(id: &Option<PresetId>) -> Option<Vec<u8>> {
-			get_preset::<RuntimeGenesisConfig>(id, self::genesis_config_presets::get_preset)
-		}
-
-		fn preset_names() -> Vec<PresetId> {
-			self::genesis_config_presets::preset_names()
-		}
-	}
-}
-
-/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
-///
-/// Other types should preferably be private.
-// TODO: this should be standardized in some way, see:
-// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
-pub mod interface {
-	use super::Runtime;
-	use polkadot_sdk::{polkadot_sdk_frame as frame, *};
-
-	pub type Block = super::Block;
-	pub use frame::runtime::types_common::OpaqueBlock;
-	pub type AccountId = <Runtime as frame_system::Config>::AccountId;
-	pub type Nonce = <Runtime as frame_system::Config>::Nonce;
-	pub type Hash = <Runtime as frame_system::Config>::Hash;
-	pub type Balance = <Runtime as pallet_balances::Config>::Balance;
-	pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
+    impl griffin_core::utxo_set::TransparentUtxoSetApi<Block> for Runtime {
+        fn peek_utxo(input: &Input) -> Option<Output> {
+            TransparentUtxoSet::peek_utxo(input)
+        }
+
+        fn peek_utxo_by_address(addr: &Address) -> Vec<Output> {
+            TransparentUtxoSet::peek_utxos_from_address(addr)
+        }
+
+        fn peek_utxo_with_asset(asset_name: &AssetName, asset_policy: &PolicyId) -> Vec<Output> {
+            TransparentUtxoSet::peek_utxos_with_asset(asset_name, asset_policy)
+        }
+    }
+
+    // https://substrate.dev/rustdocs/master/sp_api/trait.Core.html
+    impl apis::Core<Block> for Runtime {
+        fn version() -> RuntimeVersion {
+            VERSION
+        }
+
+        fn execute_block(block: Block) {
+            Executive::execute_block(block)
+        }
+
+        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
+            Executive::open_block(header)
+        }
+    }
+
+    impl apis::Metadata<Block> for Runtime {
+        fn metadata() -> OpaqueMetadata {
+            OpaqueMetadata::new(Vec::new())
+        }
+
+        fn metadata_at_version(_version: u32) -> Option<OpaqueMetadata> {
+            None
+        }
+
+        fn metadata_versions() -> alloc::vec::Vec<u32> {
+            Default::default()
+        }
+    }
+
+    impl apis::BlockBuilder<Block> for Runtime {
+        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
+            Executive::apply_extrinsic(extrinsic)
+        }
+
+        fn finalize_block() -> <Block as BlockT>::Header {
+            Executive::close_block()
+        }
+
+        fn inherent_extrinsics(_data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
+            Vec::new()
+        }
+
+        fn check_inherents(
+            _block: Block,
+            _data: InherentData
+        ) -> sp_inherents::CheckInherentsResult {
+            sp_inherents::CheckInherentsResult::new()
+        }
+    }
+
+    impl apis::TaggedTransactionQueue<Block> for Runtime {
+        fn validate_transaction(
+            source: TransactionSource,
+            tx: <Block as BlockT>::Extrinsic,
+            block_hash: <Block as BlockT>::Hash,
+        ) -> TransactionValidity {
+            Executive::validate_transaction(source, tx, block_hash)
+        }
+    }
+
+    impl apis::SessionKeys<Block> for Runtime {
+        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
+            opaque::SessionKeys::generate(seed)
+        }
+
+        fn decode_session_keys(
+            encoded: Vec<u8>,
+        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
+            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
+        }
+    }
+
+    impl apis::AuraApi<Block, AuraId> for Runtime {
+        fn slot_duration() -> sp_consensus_aura::SlotDuration {
+            sp_consensus_aura::SlotDuration::from_millis(MILLI_SECS_PER_SLOT.into())
+        }
+
+        fn authorities() -> Vec<AuraId> {
+            Authorities::aura_authorities()
+        }
+    }
+
+    impl apis::GrandpaApi<Block> for Runtime {
+        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
+            Authorities::grandpa_authorities()
+        }
+
+        fn current_set_id() -> sp_consensus_grandpa::SetId {
+            0u64
+        }
+
+        fn submit_report_equivocation_unsigned_extrinsic(
+            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
+                <Block as BlockT>::Hash,
+                sp_runtime::traits::NumberFor<Block>,
+            >,
+            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
+        ) -> Option<()> {
+            None
+        }
+
+        fn generate_key_ownership_proof(
+            _set_id: sp_consensus_grandpa::SetId,
+            _authority_id: sp_consensus_grandpa::AuthorityId,
+        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
+            None
+        }
+    }
+
+    impl apis::GenesisBuilder<Block> for Runtime {
+        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
+            let genesis_config = serde_json::from_slice::<GenesisConfig>(config.as_slice())
+                .map_err(|_| "The input JSON is not a valid genesis configuration.")?;
+
+            griffin_core::genesis::GriffinGenesisConfigBuilder::build(genesis_config)
+        }
+
+        fn get_preset(_id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
+            let genesis_config : &GenesisConfig = &genesis::get_genesis_config("".to_string());
+            Some(serde_json::to_vec(genesis_config)
+                 .expect("Genesis configuration is valid."))
+        }
+
+        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
+            vec![]
+        }
+    }
 }
```
</details>

#### Node sources

There are fewer modifications to the client, but distributed across several files.

In [chain_spec](../node/src/chain_spec.rs), we redefine the functions that build the chain from the specification:
- Import `get_genesis_config` from runtime genesis, and `WASM_BINARY ` from runtime.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/chain_spec.rs#L1-L2

- Modify `development_chain_spec()` to take a String as an argument and add the logic that uses it. The name was changed to reflect the purpose of the function more accurately.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/chain_spec.rs#L8-L18

- Add a new function for the configuration of a local test chain.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/chain_spec.rs#L20-L30

In [cli](../node/src/cli.rs):
- Add new `ExportChainSpec` command and add deprecated warning to `BuildSpec` command.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/cli.rs#L45-L51

- Modify `load_spec` function to use the new config functions defined in `chain_spec.rs`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/command.rs#L34-L44

- Add new `ExportChainSpec` command and a deprecated warning to `BuildSpec`. 

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/command.rs#L70-L73

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/command.rs#L53-L57

- Provide Griffin's `OpaqueBlock` type in `NetworkWorker`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/command.rs#L132-L137

In [service](../node/src/service.rs):

- Import `GriffinGenesisBlockBuilder` and `OpaqueBlock` as `Block` from Griffin.
- Import `self` and `RuntimeApi` from our runtime (necessary if the runtime name changed).

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L4-L5

- Within `new_partial`:
    - Define a new backend using `sc_service::new_db_backend`.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L51

    - Define `genesis_block_builder` from `GriffinGenesisBlockBuilder`.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L52-L57

    - Modify the creation of the initial parts of the node to use our custom genesis block builder.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L59-L67

    - Delete `offchain_worker` definition, as Griffin’s executive module doesn’t implement it.

- Within `new_full`:    
    - Define `chain_spec` and its new way of parsing the json.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L168-L170

    - Define `zero_time` for the ledger.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L171-L173

    - Sleep until reaching zero time for the genesis of the chain.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/service.rs#L204-L212

In [rpc](../node/src/rpc.rs):

- Import `CardanoRpc` and `CardanoRpcApiServer` from Cardano RPC within Griffin RPC.
- Import `TransparentUtxoSetRpc` and `TransparentUtxoSetRpcApiServer` from RPC within Griffin RPC.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/rpc.rs#L8-L9

- Add TransparentUtxoSetApi dependency to `create_full` function.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/rpc.rs#L41-L43

- Add the new RPC modules in the `create_full` function.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/67c4953149fb6f6d8d8c1978fcbe2c6ebab9a6ec/node/src/rpc.rs#L46-L50

<details>
  <summary>

**Complete `node/` diff** (click to expand)
  </summary>

``` diff
diff --git a/node/build.rs b/node/build.rs
--- a/node/build.rs
+++ b/node/build.rs
@@ -15,7 +15,7 @@
 // See the License for the specific language governing permissions and
 // limitations under the License.
 
-use polkadot_sdk::substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};
+use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};
 
 fn main() {
     generate_cargo_keys();
diff --git a/node/src/chain_spec.rs b/node/src/chain_spec.rs
--- a/node/src/chain_spec.rs
+++ b/node/src/chain_spec.rs
@@ -15,31 +15,33 @@
 // See the License for the specific language governing permissions and
 // limitations under the License.
 
+use minimal_template_runtime::genesis::get_genesis_config;
 use minimal_template_runtime::WASM_BINARY;
-use polkadot_sdk::{
-    sc_service::{ChainType, Properties},
-    *,
-};
+use sc_service::ChainType;
 
 /// This is a specialization of the general Substrate ChainSpec type.
 pub type ChainSpec = sc_service::GenericChainSpec;
 
-fn props() -> Properties {
-    let mut properties = Properties::new();
-    properties.insert("tokenDecimals".to_string(), 0.into());
-    properties.insert("tokenSymbol".to_string(), "MINI".into());
-    properties
-}
-
-pub fn development_chain_spec() -> Result<ChainSpec, String> {
+pub fn development_config(genesis_json: String) -> Result<ChainSpec, String> {
     Ok(ChainSpec::builder(
-        WASM_BINARY.expect("Development wasm not available"),
-        Default::default(),
+        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
+        None,
     )
     .with_name("Development")
     .with_id("dev")
     .with_chain_type(ChainType::Development)
-    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
-    .with_properties(props())
+    .with_genesis_config_patch(serde_json::json!(get_genesis_config(genesis_json)))
+    .build())
+}
+
+pub fn local_testnet_config(genesis_json: String) -> Result<ChainSpec, String> {
+    Ok(ChainSpec::builder(
+        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
+        None,
+    )
+    .with_name("Local Testnet")
+    .with_id("local_testnet")
+    .with_chain_type(ChainType::Local)
+    .with_genesis_config_patch(serde_json::json!(get_genesis_config(genesis_json)))
     .build())
 }
diff --git a/node/src/cli.rs b/node/src/cli.rs
--- a/node/src/cli.rs
+++ b/node/src/cli.rs
@@ -15,8 +15,6 @@
 // See the License for the specific language governing permissions and
 // limitations under the License.
 
-use polkadot_sdk::{sc_cli::RunCmd, *};
-
 #[derive(Debug, Clone)]
 pub enum Consensus {
     ManualSeal(u64),
@@ -49,7 +47,7 @@ pub struct Cli {
     pub consensus: Consensus,
 
     #[clap(flatten)]
-    pub run: RunCmd,
+    pub run: sc_cli::RunCmd,
 }
 
 #[derive(Debug, clap::Subcommand)]
@@ -59,8 +57,16 @@ pub enum Subcommand {
     Key(sc_cli::KeySubcommand),
 
     /// Build a chain specification.
+    /// DEPRECATED: `build-spec` command will be removed after 1/04/2026. Use `export-chain-spec`
+    /// command instead.
+    #[deprecated(
+        note = "build-spec command will be removed after 1/04/2026. Use export-chain-spec command instead"
+    )]
     BuildSpec(sc_cli::BuildSpecCmd),
 
+    /// Export the chain specification.
+    ExportChainSpec(sc_cli::ExportChainSpecCmd),
+
     /// Validate blocks.
     CheckBlock(sc_cli::CheckBlockCmd),
 
diff --git a/node/src/command.rs b/node/src/command.rs
--- a/node/src/command.rs
+++ b/node/src/command.rs
@@ -20,7 +20,8 @@ use crate::{
     cli::{Cli, Subcommand},
     service,
 };
-use polkadot_sdk::{sc_cli::SubstrateCli, sc_service::PartialComponents, *};
+use sc_cli::SubstrateCli;
+use sc_service::PartialComponents;
 
 impl SubstrateCli for Cli {
     fn impl_name() -> String {
@@ -49,10 +50,13 @@ impl SubstrateCli for Cli {
 
     fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
         Ok(match id {
-            "dev" => Box::new(chain_spec::development_chain_spec()?),
-            path => Box::new(chain_spec::ChainSpec::from_json_file(
-                std::path::PathBuf::from(path),
-            )?),
+            "dev" => Box::new(chain_spec::development_config("".to_string())?),
+            "" | "local" => Box::new(chain_spec::local_testnet_config("".to_string())?),
+            path => {
+                let file_content =
+                    std::fs::read_to_string(path).expect("Unable to read the initialization file");
+                Box::new(chain_spec::local_testnet_config(file_content)?)
+            }
         })
     }
 }
@@ -63,6 +67,7 @@ pub fn run() -> sc_cli::Result<()> {
 
     match &cli.subcommand {
         Some(Subcommand::Key(cmd)) => cmd.run(&cli),
+        #[allow(deprecated)]
         Some(Subcommand::BuildSpec(cmd)) => {
             let runner = cli.create_runner(cmd)?;
             runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
@@ -79,6 +84,10 @@ pub fn run() -> sc_cli::Result<()> {
                 Ok((cmd.run(client, import_queue), task_manager))
             })
         }
+        Some(Subcommand::ExportChainSpec(cmd)) => {
+            let chain_spec = cli.load_spec(&cmd.chain)?;
+            cmd.run(chain_spec)
+        }
         Some(Subcommand::ExportBlocks(cmd)) => {
             let runner = cli.create_runner(cmd)?;
             runner.async_run(|config| {
@@ -131,18 +140,21 @@ pub fn run() -> sc_cli::Result<()> {
         }
         Some(Subcommand::ChainInfo(cmd)) => {
             let runner = cli.create_runner(cmd)?;
-            runner.sync_run(|config| {
-                cmd.run::<minimal_template_runtime::interface::OpaqueBlock>(&config)
-            })
+            runner.sync_run(|config| cmd.run::<minimal_template_runtime::Block>(&config))
         }
         None => {
             let runner = cli.create_runner(&cli.run)?;
             runner.run_node_until_exit(|config| async move {
-                match config.network.network_backend.unwrap_or_default() {
-                    sc_network::config::NetworkBackendType::Libp2p => {
-                        service::new_full::<sc_network::NetworkWorker<_, _>>(config, cli.consensus)
-                            .map_err(sc_cli::Error::Service)
-                    }
+                match config.network.network_backend {
+                    sc_network::config::NetworkBackendType::Libp2p => service::new_full::<
+                        sc_network::NetworkWorker<
+                            griffin_core::types::OpaqueBlock,
+                            <griffin_core::types::OpaqueBlock as sp_runtime::traits::Block>::Hash,
+                        >,
+                    >(
+                        config, cli.consensus
+                    )
+                    .map_err(sc_cli::Error::Service),
                     sc_network::config::NetworkBackendType::Litep2p => service::new_full::<
                         sc_network::Litep2pNetworkBackend,
                     >(
diff --git a/node/src/main.rs b/node/src/main.rs
--- a/node/src/main.rs
+++ b/node/src/main.rs
@@ -24,6 +24,6 @@ mod command;
 mod rpc;
 mod service;
 
-fn main() -> polkadot_sdk::sc_cli::Result<()> {
+fn main() -> sc_cli::Result<()> {
     command::run()
 }
diff --git a/node/src/rpc.rs b/node/src/rpc.rs
--- a/node/src/rpc.rs
+++ b/node/src/rpc.rs
@@ -22,13 +22,12 @@
 
 #![warn(missing_docs)]
 
+use griffin_rpc::cardano_rpc::{CardanoRpc, CardanoRpcApiServer};
+use griffin_rpc::rpc::{TransparentUtxoSetRpc, TransparentUtxoSetRpcApiServer};
 use jsonrpsee::RpcModule;
-use minimal_template_runtime::interface::{AccountId, Nonce, OpaqueBlock};
-use polkadot_sdk::{
-    sc_transaction_pool_api::TransactionPool,
-    sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata},
-    *,
-};
+use sc_transaction_pool_api::TransactionPool;
+use sp_block_builder::BlockBuilder;
+use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
 use std::sync::Arc;
 
 /// Full client dependencies.
@@ -48,19 +47,24 @@ where
     C: Send
         + Sync
         + 'static
-        + sp_api::ProvideRuntimeApi<OpaqueBlock>
-        + HeaderBackend<OpaqueBlock>
-        + HeaderMetadata<OpaqueBlock, Error = BlockChainError>
+        + sp_api::ProvideRuntimeApi<<P as sc_transaction_pool_api::TransactionPool>::Block>
+        + HeaderBackend<<P as sc_transaction_pool_api::TransactionPool>::Block>
+        + HeaderMetadata<
+            <P as sc_transaction_pool_api::TransactionPool>::Block,
+            Error = BlockChainError,
+        >
         + 'static,
-    C::Api: sp_block_builder::BlockBuilder<OpaqueBlock>,
-    C::Api: substrate_frame_rpc_system::AccountNonceApi<OpaqueBlock, AccountId, Nonce>,
+    C::Api: BlockBuilder<<P as sc_transaction_pool_api::TransactionPool>::Block>,
+    C::Api: griffin_core::utxo_set::TransparentUtxoSetApi<
+        <P as sc_transaction_pool_api::TransactionPool>::Block,
+    >,
     P: TransactionPool + 'static,
 {
-    use polkadot_sdk::substrate_frame_rpc_system::{System, SystemApiServer};
     let mut module = RpcModule::new(());
     let FullDeps { client, pool } = deps;
 
-    module.merge(System::new(client.clone(), pool.clone()).into_rpc())?;
+    module.merge(CardanoRpc::new(client.clone(), pool.clone()).into_rpc())?;
+    module.merge(TransparentUtxoSetRpc::new(client.clone()).into_rpc())?;
 
     Ok(module)
 }
diff --git a/node/src/service.rs b/node/src/service.rs
--- a/node/src/service.rs
+++ b/node/src/service.rs
@@ -16,18 +16,19 @@
 // limitations under the License.
 
 use crate::cli::Consensus;
-use futures::FutureExt;
-use minimal_template_runtime::{interface::OpaqueBlock as Block, RuntimeApi};
-use polkadot_sdk::{
-    sc_client_api::backend::Backend,
-    sc_executor::WasmExecutor,
-    sc_service::{error::Error as ServiceError, Configuration, TaskManager},
-    sc_telemetry::{Telemetry, TelemetryWorker},
-    sc_transaction_pool_api::OffchainTransactionPoolFactory,
-    sp_runtime::traits::Block as BlockT,
-    *,
+use griffin_core::{genesis::GriffinGenesisBlockBuilder, types::OpaqueBlock as Block};
+use minimal_template_runtime::{self, RuntimeApi};
+use sc_executor::WasmExecutor;
+use sc_network::peer_store::LOG_TARGET;
+use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
+use sc_telemetry::{log, Telemetry, TelemetryWorker};
+use sp_runtime::traits::Block as BlockT;
+use std::{
+    sync::Arc,
+    thread::sleep,
+    time::Duration,
+    time::{SystemTime, UNIX_EPOCH},
 };
-use std::sync::Arc;
 
 type HostFunctions = sp_io::SubstrateHostFunctions;
 
@@ -62,11 +63,22 @@ pub fn new_partial(config: &Configuration) -> Result<Service, ServiceError> {
 
     let executor = sc_service::new_wasm_executor(&config.executor);
 
+    let backend = sc_service::new_db_backend(config.db_config())?;
+    let genesis_block_builder = GriffinGenesisBlockBuilder::new(
+        config.chain_spec.as_storage_builder(),
+        !config.no_genesis(),
+        backend.clone(),
+        executor.clone(),
+    )?;
+
     let (client, backend, keystore_container, task_manager) =
-        sc_service::new_full_parts::<Block, RuntimeApi, _>(
+        sc_service::new_full_parts_with_genesis_builder::<Block, RuntimeApi, _, _>(
             config,
             telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
             executor,
+            backend,
+            genesis_block_builder,
+            false,
         )?;
     let client = Arc::new(client);
 
@@ -153,29 +165,6 @@ pub fn new_full<Network: sc_network::NetworkBackend<Block, <Block as BlockT>::Ha
             metrics,
         })?;
 
-    if config.offchain_worker.enabled {
-        let offchain_workers =
-            sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
-                runtime_api_provider: client.clone(),
-                is_validator: config.role.is_authority(),
-                keystore: Some(keystore_container.keystore()),
-                offchain_db: backend.offchain_storage(),
-                transaction_pool: Some(OffchainTransactionPoolFactory::new(
-                    transaction_pool.clone(),
-                )),
-                network_provider: Arc::new(network.clone()),
-                enable_http_requests: true,
-                custom_extensions: |_| vec![],
-            })?;
-        task_manager.spawn_handle().spawn(
-            "offchain-workers-runner",
-            "offchain-worker",
-            offchain_workers
-                .run(client.clone(), task_manager.spawn_handle())
-                .boxed(),
-        );
-    }
-
     let rpc_extensions_builder = {
         let client = client.clone();
         let pool = transaction_pool.clone();
@@ -191,6 +180,19 @@ pub fn new_full<Network: sc_network::NetworkBackend<Block, <Block as BlockT>::Ha
 
     let prometheus_registry = config.prometheus_registry().cloned();
 
+    let chain_spec =
+        &serde_json::from_str::<serde_json::Value>(&config.chain_spec.as_json(false).unwrap())
+            .unwrap();
+    let zero_time = chain_spec["genesis"]["runtimeGenesis"]["patch"]["zero_time"]
+        .as_u64()
+        .unwrap();
+
+    log::warn!(
+        target: LOG_TARGET,
+        "Genesis posix time (milliseconds): {}",
+        zero_time
+    );
+
     let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
         network,
         client: client.clone(),
@@ -214,6 +216,16 @@ pub fn new_full<Network: sc_network::NetworkBackend<Block, <Block as BlockT>::Ha
         telemetry.as_ref().map(|x| x.handle()),
     );
 
+    let now = SystemTime::now()
+        .duration_since(UNIX_EPOCH)
+        .unwrap()
+        .as_millis() as u64;
+
+    // Wait until genesis time
+    sleep(Duration::from_millis(
+        zero_time.checked_sub(now).unwrap_or(0),
+    ));
+
     match consensus {
         Consensus::InstantSeal => {
             let params = sc_consensus_manual_seal::InstantSealParams {
```
</details>

### Partnerchain integration

## Operating instructions

## Troubleshooting

These are some common errors that can happen when developing on Substrate:

### `std` related issues

Errors like:

- ``Double lang item in crate <crate> (which `std`/ `serde` depends on):...``
- `Attempted to define built-in macro more than once`,

happen commonly when using std crates in a non-std environment, like Substrate's runtime. `std` crates can't be used because we compile to WASM. If you run into an error like this and the crate you are using is no-std, make sure you are setting them up correctly. For example, make sure that the dependency is imported with `default-features = false` or that the `std` feature is set correctly in the respective `Cargo.toml`. If you are writing a new module, make sure that it is premised by `#![cfg_attr(not(feature = "std"), no_std)]`.

### `alloc` feature

When trying to use `alloc` features like `vec`, you might run into the trouble that the compiler can't find the `alloc` crate. This feature can be imported from various dependencies like `serde` and `serde_json`. To use it make sure to add `extern crate alloc;` at the top of your file.

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
