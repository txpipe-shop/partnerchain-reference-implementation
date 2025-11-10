# Partnerchain integration

| Previous                                    | Next                                                | Up                         |
|---------------------------------------------|-----------------------------------------------------|----------------------------|
| [Node customization](node_customization.md) | [Operating instructions](operating_instructions.md) | [Tutorial root](README.md) |

## Dependencies

### Workspace dependencies

We must now add some elements from the PC SDK toolkit to the root of our already edited Minimal
Template. We copy them, respecting the directory structure:

```
toolkit/cli/
toolkit/committee-selection/authority-selection-inherents/
toolkit/committee-selection/primitives/
toolkit/committee-selection/query/
toolkit/committee-selection/selection/
toolkit/partner-chains-cli/
toolkit/sidechain/domain/
toolkit/sidechain/primitives/
toolkit/sidechain/sidechain-block-search/
toolkit/sidechain/sidechain-slots/
toolkit/smart-contracts/
toolkit/utils/
```

These packages must be declared at the root's Cargo, and their dependencies satisfied.

<details>
  <summary>

**Complete `[TEMPLATE_ROOT]/Cargo.toml` diff** (click to expand)
  </summary>

``` diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,25 @@ members = [
     "griffin-core",
     "griffin-rpc",
     "wallet",
+    "toolkit/cli/commands",
+    "toolkit/cli/node-commands",
+    "toolkit/committee-selection/authority-selection-inherents",
+    "toolkit/committee-selection/primitives",
+    "toolkit/committee-selection/query",
+    "toolkit/committee-selection/selection",
+    "toolkit/partner-chains-cli",
+    "toolkit/sidechain/domain",
+    "toolkit/sidechain/primitives",
+    "toolkit/sidechain/sidechain-block-search",
+    "toolkit/sidechain/sidechain-slots",
+    "toolkit/smart-contracts/commands",
+    "toolkit/smart-contracts/offchain",
+    "toolkit/smart-contracts/plutus-data",
+    "toolkit/utils/byte-string-derivation",
+    "toolkit/utils/ogmios-client",
+    "toolkit/utils/plutus",
+    "toolkit/utils/plutus/plutus-datum-derive",
+    "toolkit/utils/time-source",
 ]
 resolver = "2"
 
@@ -95,6 +114,86 @@ sc-chain-spec = { git = "https://github.com/paritytech/polkadot-sdk.git", tag =
 # demo implementations
 demo-authorities = { path = "demo/authorities", default-features = false }
 
+# Partnerchain SDK dependencies
+anyhow = "1.0.81"
+assert_cmd = "2.0.14"
+async-trait = "0.1"
+bech32 = { version = "0.11.0", default-features = false }
+blake2b_simd = { version = "1.0.2", default-features = false }
+cardano-serialization-lib = { default-features = false, version = "14.1.2" }
+cbor_event = { version = "2.4.0" }
+colored = { version = "3.0.0" }
+derive-where = { version = "1.2.7", default-features = false }
+derive_more = { version = "2.0.1", default-features = false }
+ed25519-zebra = { version = "4.0.3" }
+envy = { version = "0.4.2" }
+figment = { version = "0.10.19", features = ["env", "test"] }
+fraction = { version = "0.15.3", default-features = false }
+inquire = { version = "0.7.5" }
+itertools = "0.14.0"
+k256 = { version = "0.13.4", default-features = false }
+libp2p-identity = "0.2"
+log4rs = { version = "1.3.0" }
+minicbor = { version = "0.25.1", features = ["alloc"] }
+num-bigint = { version = "0.4.3", default-features = false }
+num-derive = { version = "0.4.2" }
+num-traits = { version = "0.2.17", default-features = false }
+once_cell = { version = "1.21.3", default-features = false }
+pallas-primitives = { version = "0.32.1" }
+pretty_assertions = { version = "1.4.1" }
+proptest = { version = "1.7.0" }
+quickcheck = { version = "1.0.3" }
+quickcheck_macros = { version = "1" }
+quote = "1.0"
+rand = { version = "0.9.1", default-features = false }
+rand_chacha = { version = "0.9.0", default-features = false }
+raw-scripts = { git = "https://github.com/input-output-hk/partner-chains-smart-contracts.git", tag = "v8.1.0" }
+secp256k1 = { version = "0.30.0", default-features = false }
+sp-consensus-slots = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-crypto-hashing = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-staking = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+sp-std = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506-2" }
+syn = "2.0"
+tempfile = "3.10.1"
+testcontainers = { version = "0.25.0" }
+time = { version = "0.3.36", default-features = false }
+tokio = { version = "1.46", features = ["rt-multi-thread", "macros"] }
+tokio-retry = { version = "0.3" }
+uplc = { version = "1.1.6" }
+
+# Local partnerchain dependencies
+
+# cli
+partner-chains-cli = { path = "toolkit/partner-chains-cli", default-features = false }
+cli-commands = { path = "toolkit/cli/commands" }
+partner-chains-node-commands = { path = "toolkit/cli/node-commands" }
+
+# committee selection
+authority-selection-inherents = { path = "toolkit/committee-selection/authority-selection-inherents", default-features = false }
+selection = { path = "toolkit/committee-selection/selection", default-features = false }
+sp-session-validator-management = { path = "toolkit/committee-selection/primitives", default-features = false }
+sp-session-validator-management-query = { path = "toolkit/committee-selection/query", default-features = false }
+
+# sidechain
+sidechain-block-search = { path = "toolkit/sidechain/sidechain-block-search", default-features = false }
+sidechain-domain = { path = "toolkit/sidechain/domain", default-features = false }
+sidechain-slots = { path = "toolkit/sidechain/sidechain-slots", default-features = false }
+sp-sidechain = { path = "toolkit/sidechain/primitives", default-features = false }
+
+# smart contracts
+partner-chains-smart-contracts-commands = { default-features = false, path = "toolkit/smart-contracts/commands"}
+partner-chains-cardano-offchain = { default-features = false, path = "toolkit/smart-contracts/offchain"}
+partner-chains-plutus-data = { default-features = false, path = "toolkit/smart-contracts/plutus-data"}
+
+# utils
+db-sync-sqlx = { path = "toolkit/utils/db-sync-sqlx" }
+byte-string-derive = { default-features = false, path = "toolkit/utils/byte-string-derivation" }
+ogmios-client = { path = "toolkit/utils/ogmios-client", default-features = false }
+plutus = { path = "toolkit/utils/plutus", default-features = false }
+plutus-datum-derive = { default-features = false, path = "toolkit/utils/plutus/plutus-datum-derive" }
+time-source = { path = "toolkit/utils/time-source" }
+
+
 [profile.release]
 opt-level = 3
 panic = "unwind"
```
</details>

#### Package dependencies

Next, the following packages will be required at the runtime:

``` diff
diff --git a/runtime/Cargo.toml b/runtime/Cargo.toml
--- a/runtime/Cargo.toml
+++ b/runtime/Cargo.toml
@@ -30,6 +30,9 @@ sp-transaction-pool = { workspace = true }
 sp-version = { workspace = true }
 
 serde = { workspace = true }
+sp-std = { workspace = true, default-features = false }
+sidechain-domain = { workspace = true, features = ["serde"] }
+authority-selection-inherents = { workspace = true }
 
 [build-dependencies]
 substrate-wasm-builder = { optional = true, workspace = true, default-features = true }
@@ -47,4 +50,7 @@ std = [
 	"sp-session/std",
 	"sp-transaction-pool/std",
 	"substrate-wasm-builder",
+	"authority-selection-inherents/std",
+	"sidechain-domain/std",
+	"sp-std/std",
 ]
```

and at the client:

``` diff
diff --git a/node/Cargo.toml b/node/Cargo.toml
index dacd46b..94f0e0e 100644
--- a/node/Cargo.toml
+++ b/node/Cargo.toml
@@ -42,6 +42,9 @@ sp-io = { workspace = true }
 sp-timestamp = { workspace = true }
 sp-runtime = { workspace = true }
 
+partner-chains-node-commands = { workspace = true }
+partner-chains-cli = { workspace = true }
+
 [build-dependencies]
 substrate-build-script-utils = { workspace = true, default-features = true }
```

FRAME deps should be eliminated from both `commands` at `cli` and `partner-chains-cli`; at the
latter, we also add a required Griffin dependency:

``` diff
diff --git a/toolkit/cli/commands/Cargo.toml b/toolkit/cli/commands/Cargo.toml
--- a/toolkit/cli/commands/Cargo.toml
+++ b/toolkit/cli/commands/Cargo.toml
@@ -31,9 +31,7 @@ sp-blockchain = { workspace = true }
 thiserror = { workspace = true }
 serde = { workspace = true }
 serde_json = { workspace = true }
-pallet-address-associations = { workspace = true, features = ["std"] }
 parity-scale-codec = { workspace = true }
-sp-block-producer-metadata = { workspace = true, features = ["std"] }
 time-source = { workspace = true }
 
 [dev-dependencies]
```

``` diff
diff --git a/toolkit/partner-chains-cli/Cargo.toml b/toolkit/partner-chains-cli/Cargo.toml
--- a/toolkit/partner-chains-cli/Cargo.toml
+++ b/toolkit/partner-chains-cli/Cargo.toml
@@ -39,15 +39,10 @@ plutus = { workspace = true }
 plutus-datum-derive = { workspace = true }
 ed25519-zebra = { workspace = true }
 sp-session-validator-management = { workspace = true, features = ["std"] }
-pallet-session-validator-management = { workspace = true, features = ["std"] }
-pallet-partner-chains-bridge = { workspace = true, features = ["std"] }
-sp-partner-chains-bridge = { workspace = true, features = ["std"] }
-pallet-partner-chains-session = { workspace = true, features = ["std"] }
-pallet-sidechain = { workspace = true, features = ["std"] }
-pallet-governed-map = { workspace = true, features = ["std"] }
-sp-governed-map = { workspace = true, features = ["std"] }
 sidechain-slots = { workspace = true }
 authority-selection-inherents = { workspace = true, features = ["std"] }
+griffin-core = { workspace = true }
+partner-chains-plutus-data = { workspace = true }
 
 [dev-dependencies]
 frame-system = { workspace = true }
```

## Tweaking the PC SDK

### Modifications at the runtime sources

| Previous                                    | Next                                                | Up                         |
|---------------------------------------------|-----------------------------------------------------|----------------------------|
| [Node customization](node_customization.md) | [Operating instructions](operating_instructions.md) | [Tutorial root](README.md) |

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
