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
+
 [profile.release]
 opt-level = 3
 panic = "unwind"

```

In this project we decided to steer away from using `polkadot-sdk` as one big dependency. Instead,
we pick and choose what we need from the Polkadot-SDK and set each as their own dependency, using
the Git tag for release `polkadot-stable2506-2` (click to see the rather long diff):

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
 codec = { version = "3.7.4", default-features = false, package = "parity-scale-codec" }
 scale-info = { version = "2.11.6", default-features = false }
 serde_json = { version = "1.0.132", default-features = false }
```
<details>


<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
