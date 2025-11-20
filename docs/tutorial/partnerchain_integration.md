# Partnerchain integration

| Previous                                    | Next                                            | Up                         |
|---------------------------------------------|-------------------------------------------------|----------------------------|
| [Node customization](node_customization.md) | [Use-case application](use_case_application.md) | [Tutorial root](README.md) |

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

These packages must be declared at the root's Cargo, and their dependencies satisfied. An
appropriate version number should also be defined: PC `toolkit` packages expect one.

<details>
  <summary>

*Complete `[TEMPLATE_ROOT]/Cargo.toml` diff* (click to expand)
  </summary>

``` diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -4,6 +4,7 @@ authors = ["Parity Technologies <admin@parity.io>"]
 homepage = "https://paritytech.github.io/polkadot-sdk/"
 repository = "https://github.com/paritytech/polkadot-sdk-minimal-template.git"
 edition = "2021"
+version = "0.1.0"
 
 [workspace]
 default-members = ["runtime"]
@@ -14,6 +15,25 @@ members = [
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
 
@@ -95,6 +115,86 @@ sc-chain-spec = { git = "https://github.com/paritytech/polkadot-sdk.git", tag =
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
 
diff --git a/runtime/Cargo.toml b/runtime/Cargo.toml
index d66f10d..4a48a8a 100644
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
diff --git a/toolkit/cli/commands/Cargo.toml b/toolkit/cli/commands/Cargo.toml
index 404ab1e..6ffd584 100644
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
diff --git a/toolkit/partner-chains-cli/Cargo.toml b/toolkit/partner-chains-cli/Cargo.toml
index 1c8cd74..76f75b8 100644
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
</details>

### Package dependencies

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

## Integrating the Partnerchain SDK

### PC SDK toolkit tweaks

In the following packages, we needed to remove some functionalities that we won’t
support. There are many changes to the original SDK here so if you’re following our pallet-less
approach it might be more convenient to just copy from our source code. Nevertheless, the changes
have been made explicit below.

#### `partner-chains-cli`

We need to modify the `create_chain_spec` module, obtaining the authorities from the Genesis:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/9bafbcac2b3fe483e6fb70faa443942c6f281b9c/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs#L31-L47

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/9bafbcac2b3fe483e6fb70faa443942c6f281b9c/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs#L189-L229

We also have to remove references to FRAME pallets, and add necessary deps:

<details>
  <summary>

*Removals and imports at `create_chain_spec/mod.rs`* (click to expand)
  </summary>

``` diff
diff --git a/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs b/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs
--- a/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs
+++ b/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs
@@ -5,9 +5,10 @@ use crate::runtime_bindings::PartnerChainRuntime;
 use crate::{config::config_fields, CmdRun};
 use anyhow::anyhow;
 use authority_selection_inherents::MaybeFromCandidateKeys;
+use griffin_core::genesis::config_builder::GenesisConfig;
+use partner_chains_plutus_data::permissioned_candidates::permissioned_candidates_to_plutus_data;
 use sidechain_domain::{AssetName, MainchainAddress, PolicyId, UtxoId};
-use sp_core::ecdsa;
-use sp_runtime::{AccountId32, DeserializeOwned};
+use sp_runtime::DeserializeOwned;
 use std::marker::PhantomData;
 
 #[cfg(test)]
@@ -43,57 +44,6 @@ impl<T: PartnerChainRuntime> CreateChainSpecCmd<T> {
     fn print_config<C: IOContext>(context: &C, config: &CreateChainSpecConfig<T::Keys>) {
         context.print("Chain parameters:");
         context.print(format!("- Genesis UTXO: {}", config.genesis_utxo).as_str());
-        context.print("SessionValidatorManagement Main Chain Configuration:");
-        context.print(
-            format!(
-                "- committee_candidate_address: {}",
-                config.committee_candidate_address
-            )
-            .as_str(),
-        );
-        context.print(
-            format!(
-                "- d_parameter_policy_id: {}",
-                config.d_parameter_policy_id.to_hex_string()
-            )
-            .as_str(),
-        );
-        context.print(
-            format!(
-                "- permissioned_candidates_policy_id: {}",
-                config.permissioned_candidates_policy_id.to_hex_string()
-            )
-            .as_str(),
-        );
-        context.print("Bridge Configuration (unused if empty):");
-        context.print(&format!(
-            "- asset name: {}",
-            config.bridge_token_asset_name.to_hex_string()
-        ));
-        context.print(&format!(
-            "- asset policy ID: {}",
-            config.bridge_token_policy.to_hex_string()
-        ));
-        context.print(&format!(
-            "- illiquid circulation supply validator address: {}",
-            config.illiquid_circulation_supply_validator_address
-        ));
-        context.print("Governed Map Configuration:");
-        context.print(&format!(
-            "- validator address: {}",
-            config
-                .governed_map_validator_address
-                .clone()
-                .unwrap_or_default()
-        ));
-        context.print(&format!(
-            "- asset policy ID: {}",
-            config
-                .governed_map_asset_policy_id
-                .clone()
-                .unwrap_or_default()
-                .to_hex_string()
-        ));
         use colored::Colorize;
         if config.initial_permissioned_candidates_parsed.is_empty() {
             context.print("WARNING: The list of initial permissioned candidates is empty. Generated chain spec will not allow the chain to start.".red().to_string().as_str());
@@ -168,106 +118,6 @@ impl<Keys: MaybeFromCandidateKeys> CreateChainSpecConfig<Keys> {
             governed_map_asset_policy_id: config_fields::GOVERNED_MAP_POLICY_ID.load_from_file(c),
         })
     }
-
-    /// Returns [pallet_sidechain::GenesisConfig] derived from the config
-    pub fn pallet_sidechain_config<T: pallet_sidechain::Config>(
-        &self,
-        slots_per_epoch: sidechain_slots::SlotsPerEpoch,
-    ) -> pallet_sidechain::GenesisConfig<T> {
-        pallet_sidechain::GenesisConfig {
-            genesis_utxo: self.genesis_utxo,
-            slots_per_epoch,
-            _config: PhantomData,
-        }
-    }
-
-    /// Returns [pallet_partner_chains_session::GenesisConfig] derived from the config, using initial permissioned candidates
-    /// as initial validators
-    pub fn pallet_partner_chains_session_config<T: pallet_partner_chains_session::Config>(
-        &self,
-    ) -> pallet_partner_chains_session::GenesisConfig<T>
-    where
-        T::ValidatorId: From<AccountId32>,
-        T::Keys: From<Keys>,
-    {
-        pallet_partner_chains_session::GenesisConfig {
-            initial_validators: self
-                .initial_permissioned_candidates_parsed
-                .iter()
-                .map(|c| (c.account_id_32().into(), c.keys.clone().into()))
-                .collect::<Vec<_>>(),
-        }
-    }
-
-    /// Returns [pallet_session_validator_management::GenesisConfig] derived from the config using initial permissioned candidates
-    /// as initial authorities
-    pub fn pallet_session_validator_management_config<
-        T: pallet_session_validator_management::Config,
-    >(
-        &self,
-    ) -> pallet_session_validator_management::GenesisConfig<T>
-    where
-        T::AuthorityId: From<ecdsa::Public>,
-        T::AuthorityKeys: From<Keys>,
-        T::CommitteeMember:
-            From<authority_selection_inherents::CommitteeMember<T::AuthorityId, T::AuthorityKeys>>,
-    {
-        pallet_session_validator_management::GenesisConfig {
-            initial_authorities: self
-                .initial_permissioned_candidates_parsed
-                .iter()
-                .map(|c| {
-                    authority_selection_inherents::CommitteeMember::permissioned(
-                        c.sidechain.into(),
-                        c.keys.clone().into(),
-                    )
-                    .into()
-                })
-                .collect::<Vec<_>>(),
-            main_chain_scripts: sp_session_validator_management::MainChainScripts {
-                committee_candidate_address: self.committee_candidate_address.clone(),
-                d_parameter_policy_id: self.d_parameter_policy_id.clone(),
-                permissioned_candidates_policy_id: self.permissioned_candidates_policy_id.clone(),
-            },
-        }
-    }
-
-    /// Returns [pallet_partner_chains_bridge::GenesisConfig] derived from the config
-    pub fn bridge_config<T: pallet_partner_chains_bridge::Config>(
-        &self,
-    ) -> pallet_partner_chains_bridge::GenesisConfig<T> {
-        pallet_partner_chains_bridge::GenesisConfig {
-            main_chain_scripts: Some(sp_partner_chains_bridge::MainChainScripts {
-                token_policy_id: self.bridge_token_policy.clone(),
-                token_asset_name: self.bridge_token_asset_name.clone(),
-                illiquid_circulation_supply_validator_address: self
-                    .illiquid_circulation_supply_validator_address
-                    .clone(),
-            }),
-            initial_checkpoint: Some(self.genesis_utxo),
-            _marker: PhantomData,
-        }
-    }
-
-    /// Returns [pallet_governed_map::GenesisConfig] derived from the config
-    pub fn governed_map_config<T: pallet_governed_map::Config>(
-        &self,
-    ) -> pallet_governed_map::GenesisConfig<T> {
-        pallet_governed_map::GenesisConfig {
-            main_chain_scripts: self
-                .governed_map_validator_address
-                .as_ref()
-                .and_then(|addr| {
-                    self.governed_map_asset_policy_id.as_ref().map(|policy| {
-                        sp_governed_map::MainChainScriptsV1 {
-                            validator_address: addr.clone(),
-                            asset_policy_id: policy.clone(),
-                        }
-                    })
-                }),
-            _marker: PhantomData,
-        }
-    }
 }
 
 impl<T> Default for CreateChainSpecConfig<T> {
```
</details>

<details>
  <summary>

We also fine-tuned some of CLI messages (click to expand)
  </summary>

``` diff
diff --git a/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs b/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs
--- a/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs
+++ b/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs
@@ -23,7 +23,10 @@ pub struct CreateChainSpecCmd<T: PartnerChainRuntime> {
 impl<T: PartnerChainRuntime> CmdRun for CreateChainSpecCmd<T> {
     fn run<C: IOContext>(&self, context: &C) -> anyhow::Result<()> {
         let config = CreateChainSpecConfig::load(context)?;
-        context.print("This wizard will create a chain spec JSON file according to the provided configuration, using WASM runtime code from the compiled node binary.");
+        context.print("This wizard will create a genesis.json to use as chain spec, using the candidates found in the the provided configuration");
+        context.print(
+                "If the chain includes registered candidates, you need to obtain their keys and add them to the permissioned candidates list in the configuration as well, to set up the genesis accordingly. You need to have all the candidate's keys before moving on, or else they won't be able to participate in the chain.",
+            );
         Self::print_config(context, &config);
         if context.prompt_yes_no("Do you want to continue?", true) {
             let initial_permissioned_candidates_data: &Vec<
@@ -63,9 +66,9 @@ impl<T: PartnerChainRuntime> CreateChainSpecCmd<T> {
         context.print(format!("- Genesis UTXO: {}", config.genesis_utxo).as_str());
         use colored::Colorize;
         if config.initial_permissioned_candidates_parsed.is_empty() {
-            context.print("WARNING: The list of initial permissioned candidates is empty. Generated chain spec will not allow the chain to start.".red().to_string().as_str());
+            context.print("WARNING: The list of candidates is empty. Generated chain spec will not allow the chain to start.".red().to_string().as_str());
             let update_msg = format!(
-				"Update 'initial_permissioned_candidates' field of {} file with keys of initial committee.",
+				"Update 'initial_permissioned_candidates' field of {} file with keys of the committee.",
 				context
 					.config_file_path(config_fields::INITIAL_PERMISSIONED_CANDIDATES.config_file)
 			);
@@ -77,7 +80,7 @@ impl<T: PartnerChainRuntime> CreateChainSpecCmd<T> {
                     .as_str(),
             );
         } else {
-            context.print("Initial permissioned candidates:");
+            context.print("Candidates:");
             for candidate in config.initial_permissioned_candidates_raw.iter() {
                 context.print(format!("- {}", candidate).as_str());
             }
```
</details>

#### `cli`

<details>
  <summary>

*Removals from `toolkit/cli/`* (click to expand)
  </summary>

``` diff
diff --git a/toolkit/cli/commands/src/lib.rs b/toolkit/cli/commands/src/lib.rs
--- a/toolkit/cli/commands/src/lib.rs
+++ b/toolkit/cli/commands/src/lib.rs
@@ -3,8 +3,5 @@
 //! Provides clap-based commands for signature generation, address association,
 //! and blockchain queries. Used by `partner-chains-node-commands`.
 
-pub mod address_association_signatures;
-pub mod block_producer_metadata_signatures;
-pub mod get_genesis_utxo;
 pub mod key_params;
 pub mod registration_signatures;
diff --git a/toolkit/cli/node-commands/src/lib.rs b/toolkit/cli/node-commands/src/lib.rs
index 5cbd01b..42ffde9 100644
--- a/toolkit/cli/node-commands/src/lib.rs
+++ b/toolkit/cli/node-commands/src/lib.rs
@@ -2,35 +2,15 @@
 //! and a [run] function for running these commands.
 //! [PartnerChainsSubcommand] is meant to be used by a command line argument parser library.
 #![deny(missing_docs)]
-use authority_selection_inherents::{
-    AuthoritySelectionDataSource, AuthoritySelectionInputs, CandidateValidationApi,
-};
 use clap::Parser;
-use cli_commands::address_association_signatures::AddressAssociationSignaturesCmd;
-use cli_commands::block_producer_metadata_signatures::BlockProducerMetadataSignatureCmd;
 use cli_commands::registration_signatures::RegistrationSignaturesCmd;
 use frame_support::sp_runtime::traits::NumberFor;
-use parity_scale_codec::{Decode, Encode};
 use partner_chains_cli::DefaultCmdRunContext;
 pub use partner_chains_cli::PartnerChainRuntime;
 use partner_chains_smart_contracts_commands::SmartContractsCmd;
-use sc_cli::{CliConfiguration, SharedParams, SubstrateCli};
-use sc_service::TaskManager;
+use sc_cli::{CliConfiguration, SharedParams};
 use sidechain_domain::*;
-use sp_api::ProvideRuntimeApi;
-use sp_blockchain::HeaderBackend;
 use sp_runtime::traits::Block as BlockT;
-use sp_runtime::AccountId32;
-use sp_runtime::DeserializeOwned;
-use sp_runtime::Serialize;
-use sp_session_validator_management::CommitteeMember as CommitteeMemberT;
-use sp_session_validator_management::SessionValidatorManagementApi;
-use sp_session_validator_management_query::commands::*;
-use sp_session_validator_management_query::SessionValidatorManagementQuery;
-#[allow(deprecated)]
-use sp_sidechain::{GetGenesisUtxo, GetSidechainStatus};
-use std::future::Future;
-use std::sync::Arc;
 
 #[derive(Debug, Clone, Parser)]
 /// Command line arguments for the `ariadne-parameters` command.
@@ -84,56 +64,13 @@ impl CliConfiguration for RegistrationStatusCmd {
     }
 }
 
-static REGISTRATION_STATUS_AFTER_HELP: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(
-    || {
-        fn get_node_exe_name() -> Option<String> {
-            let exe_path = std::env::current_exe().ok()?;
-            let exe_name_osstr = exe_path.file_name()?.to_os_string();
-            Some(exe_name_osstr.to_str()?.to_string())
-        }
-        format!(
-			"Example: {} registration-status --stake-pool-pub-key 0x702b81ab2e86cf73a87062af1eb0da666d451976d9d91c63a119ed94e6a33dc0 --mc-epoch-number 586",
-			get_node_exe_name().unwrap_or("node-bin".to_string())
-		)
-    },
-);
-
 #[derive(Clone, Debug, clap::Subcommand)]
 #[allow(clippy::large_enum_variant)]
 /// Entry point for all Partner Chains specific subcommand.
-pub enum PartnerChainsSubcommand<
-    RuntimeBindings: PartnerChainRuntime + Send + Sync,
-    PartnerchainAddress: Clone + Sync + Send + FromStrStdErr + 'static,
-> {
-    /// Returns sidechain parameters.
-    /// Requires --chain parameter that results in loading a properly configured chain spec.
-    SidechainParams(SidechainParamsCmd),
-
-    /// Returns registration status for a given stake pool public key and epoch number.
-    /// If registration has been included in Cardano block in epoch N, then it should be returned by this command if epoch greater than N+1 is provided.
-    /// If this command won't show your registration after a few minutes after it has been included in a cardano block, you can start debugging for unsuccessful registration.
-    /// Requires --chain parameter that results in loading a properly configured chain spec.
-    #[clap(
-		after_help = &*REGISTRATION_STATUS_AFTER_HELP
-	)]
-    RegistrationStatus(RegistrationStatusCmd),
-
-    /// Returns ariadne parameters effective at given mainchain epoch number.
-    /// Parameters are effective two epochs after the block their change is included in
-    /// Cardano.
-    /// Requires --chain parameter that results in loading a properly configured chain spec.
-    AriadneParameters(AriadneParametersCmd),
-
+pub enum PartnerChainsSubcommand<RuntimeBindings: PartnerChainRuntime + Send + Sync> {
     /// Generates registration signatures for partner chains committee candidates
     RegistrationSignatures(RegistrationSignaturesCmd),
 
-    /// Signs address association
-    SignAddressAssociation(AddressAssociationSignaturesCmd<PartnerchainAddress>),
-
-    /// Signs block producer metadata for submitting to the runtime
-    #[command(subcommand)]
-    SignBlockProducerMetadata(BlockProducerMetadataSignatureCmd<AccountId32>),
-
     /// Commands for interacting with Partner Chain smart contracts on Cardano
     #[command(subcommand)]
     SmartContracts(SmartContractsCmd),
@@ -145,95 +82,15 @@ pub enum PartnerChainsSubcommand<
 
 #[allow(deprecated)]
 /// Runs a Partner Chains subcommand.
-pub fn run<
-    Cli,
-    Block,
-    CommitteeMember,
-    Client,
-    BlockProducerMetadata,
-    RuntimeBindings: PartnerChainRuntime + Send + Sync,
-    PartnerchainAddress,
->(
-    cli: &Cli,
-    get_deps: impl FnOnce(
-        sc_service::Configuration,
-    ) -> Result<
-        (
-            Arc<Client>,
-            TaskManager,
-            Arc<dyn AuthoritySelectionDataSource + Send + Sync>,
-        ),
-        sc_service::error::Error,
-    >,
-    cmd: PartnerChainsSubcommand<RuntimeBindings, PartnerchainAddress>,
+pub fn run<Block, RuntimeBindings: PartnerChainRuntime + Send + Sync>(
+    cmd: PartnerChainsSubcommand<RuntimeBindings>, // partnerchainAddress
 ) -> sc_cli::Result<()>
 where
-    Cli: SubstrateCli,
-    Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + 'static,
-    Client::Api: GetGenesisUtxo<Block>
-        + GetSidechainStatus<Block>
-        + SessionValidatorManagementApi<
-            Block,
-            CommitteeMember,
-            AuthoritySelectionInputs,
-            ScEpochNumber,
-        > + CandidateValidationApi<Block>,
     Block: BlockT,
     NumberFor<Block>: From<u32> + Into<u32>,
-    CommitteeMember: CommitteeMemberT + Encode + Decode + Send + Sync + 'static,
-    CommitteeMember::AuthorityId: Decode + Encode + AsRef<[u8]> + Send + Sync + 'static,
-    CommitteeMember::AuthorityKeys: Decode + Encode,
-    BlockProducerMetadata: DeserializeOwned + Encode + Send + Sync,
-    PartnerchainAddress: Serialize + Clone + Sync + Send + FromStrStdErr + Encode + 'static,
 {
     match cmd {
-        PartnerChainsSubcommand::SidechainParams(cmd) => {
-            let runner = cli.create_runner(&cmd)?;
-            runner.async_run(|config| {
-                let (client, task_manager, _) = get_deps(config)?;
-                Ok((
-                    print_result(cli_commands::get_genesis_utxo::execute(client)),
-                    task_manager,
-                ))
-            })
-        }
-        PartnerChainsSubcommand::RegistrationStatus(cmd) => {
-            let runner = cli.create_runner(&cmd)?;
-            runner.async_run(move |config| {
-                let (client, task_manager, ds) = get_deps(config)?;
-                let query = SessionValidatorManagementQuery::new(client.clone(), ds.clone());
-                Ok((
-                    print_result(cli_get_registration_status(
-                        query,
-                        cmd.mc_epoch_number,
-                        cmd.stake_pool_pub_key.clone(),
-                    )),
-                    task_manager,
-                ))
-            })
-        }
-        PartnerChainsSubcommand::AriadneParameters(cmd) => {
-            let runner = cli.create_runner(&cmd)?;
-            runner.async_run(move |config| {
-                let (client, task_manager, ds) = get_deps(config)?;
-                let query = SessionValidatorManagementQuery::new(client.clone(), ds.clone());
-                Ok((
-                    print_result(cli_get_ariadne_parameters(query, cmd.mc_epoch_number)),
-                    task_manager,
-                ))
-            })
-        }
         PartnerChainsSubcommand::RegistrationSignatures(cmd) => Ok(println!("{}", cmd.execute())),
-        PartnerChainsSubcommand::SignAddressAssociation(cmd) => {
-            cmd.execute()
-                .map_err(|e| sc_service::Error::Application(e.into()))?;
-            Ok(())
-        }
-        PartnerChainsSubcommand::SignBlockProducerMetadata(cmd) => {
-            cmd.execute::<BlockProducerMetadata>()
-                .map_err(|e| sc_service::Error::Application(e.into()))?;
-            Ok(())
-        }
         PartnerChainsSubcommand::SmartContracts(cmd) => {
             setup_log4rs()?;
             Ok(cmd.execute_blocking()?)
@@ -275,15 +132,6 @@ fn setup_log4rs() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
     Ok(())
 }
 
-async fn print_result<FIn>(command_future: FIn) -> Result<(), sc_cli::Error>
-where
-    FIn: Future<Output = Result<String, String>>,
-{
-    let result = command_future.await.unwrap_or_else(|e| e);
-    println!("{}", result);
-    Ok(())
-}
-
 #[cfg(test)]
 mod tests {
```
</details>

#### `smart-contracts`

From this package, we also remove
```
toolkit/smart-contracts/commands/src/bridge.rs
toolkit/smart-contracts/commands/src/reserve.rs
```
and all references to them in the code.

<details>
  <summary>

*Removals from `toolkit/smart-contracts/`* (click to expand)
  </summary>

``` diff
diff --git a/toolkit/smart-contracts/commands/src/lib.rs b/toolkit/smart-contracts/commands/src/lib.rs
--- a/toolkit/smart-contracts/commands/src/lib.rs
+++ b/toolkit/smart-contracts/commands/src/lib.rs
@@ -33,14 +33,12 @@ use sidechain_domain::*;
 use std::time::Duration;
 
 pub mod assemble_tx;
-pub mod bridge;
 pub mod d_parameter;
 pub mod get_scripts;
 pub mod governance;
 pub mod governed_map;
 pub mod permissioned_candidates;
 pub mod register;
-pub mod reserve;
 pub mod sign_tx;
 
 #[derive(Clone, Debug, clap::Subcommand)]
@@ -58,21 +56,12 @@ pub enum SmartContractsCmd {
     /// Deregister candidate
     Deregister(register::DeregisterCmd),
     #[command(subcommand)]
-    /// Commands for management of rewards reserve
-    Reserve(reserve::ReserveCmd),
-    #[command(subcommand)]
     /// Commands for management of on-chain governance
     Governance(governance::GovernanceCmd),
     /// Assemble and submit a transaction
     AssembleAndSubmitTx(assemble_tx::AssembleAndSubmitCmd),
     /// Sign a transaction CBOR using a payment signing key
     SignTx(sign_tx::SignTxCmd),
-    #[command(subcommand)]
-    /// Manage the Governed Map key-value store on Cardano
-    GovernedMap(governed_map::GovernedMapCmd),
-    #[command(subcommand)]
-    /// Send token to bridge contract
-    Bridge(bridge::BridgeCmd),
 }
 
 #[derive(Clone, Debug, clap::Parser)]
@@ -135,11 +124,8 @@ impl SmartContractsCmd {
             Self::UpsertPermissionedCandidates(cmd) => cmd.execute().await,
             Self::Register(cmd) => cmd.execute().await,
             Self::Deregister(cmd) => cmd.execute().await,
-            Self::Reserve(cmd) => cmd.execute().await,
             Self::AssembleAndSubmitTx(cmd) => cmd.execute().await,
             Self::SignTx(cmd) => cmd.execute().await,
-            Self::GovernedMap(cmd) => cmd.execute().await,
-            Self::Bridge(cmd) => cmd.execute().await,
         }?;
         println!("{}", result);
         Ok(())
diff --git a/toolkit/smart-contracts/offchain/src/csl.rs b/toolkit/smart-contracts/offchain/src/csl.rs
--- a/toolkit/smart-contracts/offchain/src/csl.rs
+++ b/toolkit/smart-contracts/offchain/src/csl.rs
@@ -336,8 +336,6 @@ pub(crate) trait OgmiosUtxoExt {
     fn to_csl_tx_output(&self) -> Result<TransactionOutput, JsError>;
     fn to_csl(&self) -> Result<TransactionUnspentOutput, JsError>;
 
-    fn get_asset_amount(&self, asset: &AssetId) -> u64;
-
     fn get_plutus_data(&self) -> Option<PlutusData>;
 }
 
@@ -387,17 +385,6 @@ impl OgmiosUtxoExt for OgmiosUtxo {
         ))
     }
 
-    fn get_asset_amount(&self, asset_id: &AssetId) -> u64 {
-        self.value
-            .native_tokens
-            .get(&asset_id.policy_id.0)
-            .cloned()
-            .unwrap_or_default()
-            .iter()
-            .find(|asset| asset.name == asset_id.asset_name.0.to_vec())
-            .map_or_else(|| 0, |asset| asset.amount)
-    }
-
     fn get_plutus_data(&self) -> Option<PlutusData> {
         (self.datum.as_ref())
             .map(|datum| datum.bytes.clone())
@@ -905,40 +892,12 @@ impl AssetNameExt for sidechain_domain::AssetName {
     }
 }
 
-pub(crate) trait AssetIdExt {
-    fn to_multi_asset(&self, amount: impl Into<BigNum>) -> Result<MultiAsset, JsError>;
-}
-impl AssetIdExt for AssetId {
-    fn to_multi_asset(&self, amount: impl Into<BigNum>) -> Result<MultiAsset, JsError> {
-        let mut ma = MultiAsset::new();
-        let mut assets = Assets::new();
-        assets.insert(&self.asset_name.to_csl()?, &amount.into());
-        ma.insert(&self.policy_id.0.into(), &assets);
-        Ok(ma)
-    }
-}
-
 pub(crate) trait MultiAssetExt: Sized {
-    fn from_ogmios_utxo(utxo: &OgmiosUtxo) -> Result<Self, JsError>;
     fn with_asset_amount(self, asset: &AssetId, amount: impl Into<BigNum>)
         -> Result<Self, JsError>;
 }
 
 impl MultiAssetExt for MultiAsset {
-    fn from_ogmios_utxo(utxo: &OgmiosUtxo) -> Result<Self, JsError> {
-        let mut ma = MultiAsset::new();
-        for (policy, policy_assets) in utxo.value.native_tokens.iter() {
-            let mut assets = Assets::new();
-            for asset in policy_assets {
-                assets.insert(
-                    &cardano_serialization_lib::AssetName::new(asset.name.clone())?,
-                    &asset.amount.into(),
-                );
-            }
-            ma.insert(&PolicyID::from(*policy), &assets);
-        }
-        Ok(ma)
-    }
     fn with_asset_amount(
         mut self,
         asset: &AssetId,
diff --git a/toolkit/smart-contracts/offchain/src/lib.rs b/toolkit/smart-contracts/offchain/src/lib.rs
--- a/toolkit/smart-contracts/offchain/src/lib.rs
+++ b/toolkit/smart-contracts/offchain/src/lib.rs
@@ -5,8 +5,6 @@
 pub mod assemble_and_submit_tx;
 /// Primitives used for awaiting for tx being observed on the blockchain
 pub mod await_tx;
-/// Deposits to (in the future withdrawals from as well) bridge validator
-pub mod bridge;
 /// Parsing and wrapping of Cardano keys
 pub mod cardano_keys;
 /// General purpose code for interacting with cardano-serialization-lib
@@ -29,7 +27,6 @@ pub mod permissioned_candidates;
 pub mod plutus_script;
 /// Supports candidate registration
 pub mod register;
-pub mod reserve;
 /// Provides synthesized scripts data
 pub mod scripts_data;
 /// Signing transactions
@@ -39,8 +36,6 @@ mod test_values;
 /// Supports governance updates
 pub mod update_governance;
 
-mod versioning_system;
-
 /// Simply wraps asset id with amount.
 #[derive(Clone)]
 pub struct TokenAmount {
```
</details>


### Modifications at the runtime sources

At the runtime lib, we need to import `KeyTypeId` from `sp_core`,

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/38773809024d12f643407f6e2e845bb359e5a16d/runtime/src/lib.rs#L25

implement `cross_chain_app` in `opaque` module

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/38773809024d12f643407f6e2e845bb359e5a16d/runtime/src/lib.rs#L80-L123

(note that `MaybeFromCandidateKeys` in that implementation belongs to the `authority-selection-inherents` package), define `CrossChainKey` within `impl_opaque_keys` macro, and finally add the `CrossChainPublic` type alias:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/38773809024d12f643407f6e2e845bb359e5a16d/runtime/src/lib.rs#L127-L134

### Modifications at the node sources

At the client, we need to implement the `PartnerChainRuntime` trait and plug them to the former's
CLI.

<details>
  <summary>

*Implementation at `node/src/`* (click to expand)
  </summary>

``` diff
diff --git a/node/src/cli.rs b/node/src/cli.rs
--- a/node/src/cli.rs
+++ b/node/src/cli.rs
@@ -14,6 +14,9 @@
 // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 // See the License for the specific language governing permissions and
 // limitations under the License.
+use minimal_template_runtime::opaque::SessionKeys;
+use partner_chains_cli::{KeyDefinition, AURA, GRANDPA};
+use partner_chains_node_commands::{PartnerChainRuntime, PartnerChainsSubcommand};
 
 #[derive(Debug, Clone)]
 pub enum Consensus {
@@ -22,6 +25,67 @@ pub enum Consensus {
     None,
 }
 
+#[derive(Debug, Clone)]
+pub struct WizardBindings;
+
+impl PartnerChainRuntime for WizardBindings {
+    type Keys = SessionKeys;
+
+    fn key_definitions() -> Vec<KeyDefinition<'static>> {
+        vec![AURA, GRANDPA]
+    }
+
+    // This function is required by the PartnerChainsRuntime trait
+    // Leaving it empty won't work as it parses for the ChainSpec structure whichever it might be
+    // We give an implementation using the default genesis
+    fn create_chain_spec(
+        _config: &partner_chains_cli::CreateChainSpecConfig<SessionKeys>,
+    ) -> serde_json::Value {
+        let genesis_default: &str = r#"
+            {
+                "zero_time": 1747081100000,
+                "zero_slot": 0,
+                "outputs": [
+                    {
+                        "address": "6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4",
+                        "coin": 314000000,
+                        "value": [
+                                {
+                                    "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
+                                    "assets": [ ["tokenA", 271000000], ["tokenB", 1123581321] ]
+                                }
+                                ],
+                        "datum": "820080"
+                    },
+                    {
+                        "address": "61547932e40a24e2b7deb41f31af21ed57acd125f4ed8a72b626b3d7f6",
+                        "coin": 314150000,
+                        "value": [
+                                {
+                                    "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
+                                    "assets": [ ["tokenA", 300000000], ["tokenB", 2000000000] ]
+                                }
+                                ],
+                        "datum": "820080"
+                    },
+                    {
+                        "address": "0000000000000000000000000000000000000000000000000000000000",
+                        "coin": 314150000,
+                        "value": [
+                                {
+                                    "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
+                                    "assets": [ ["Authorities", 300000000]]
+                                }
+                                ],
+                        "datum": "9FD879809F9F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC5820D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D582088DC3417D5058EC4B4503E0C12EA1A0A89BE200FE98922423D4334014FA6B0EEFFFF00FF"
+                    }
+                ]
+            }
+            "#;
+        serde_json::from_str(genesis_default).unwrap()
+    }
+}
+
 impl std::str::FromStr for Consensus {
     type Err = String;
 
@@ -56,6 +120,9 @@ pub enum Subcommand {
     #[command(subcommand)]
     Key(sc_cli::KeySubcommand),
 
+    #[clap(flatten)]
+    PartnerChains(PartnerChainsSubcommand<WizardBindings>),
+
     /// Build a chain specification.
     /// DEPRECATED: `build-spec` command will be removed after 1/04/2026. Use `export-chain-spec`
     /// command instead.
diff --git a/node/src/command.rs b/node/src/command.rs
--- a/node/src/command.rs
+++ b/node/src/command.rs
@@ -17,7 +17,7 @@
 
 use crate::{
     chain_spec,
-    cli::{Cli, Subcommand},
+    cli::{Cli, Subcommand, WizardBindings},
     service,
 };
 use sc_cli::SubstrateCli;
@@ -67,6 +67,12 @@ pub fn run() -> sc_cli::Result<()> {
 
     match &cli.subcommand {
         Some(Subcommand::Key(cmd)) => cmd.run(&cli),
+        Some(Subcommand::PartnerChains(cmd)) => {
+            partner_chains_node_commands::run::<
+                griffin_core::types::OpaqueBlock,
+                WizardBindings,
+            >(cmd.clone())
+        }
         #[allow(deprecated)]
         Some(Subcommand::BuildSpec(cmd)) => {
             let runner = cli.create_runner(cmd)?;
```
</details>

| Previous                                    | Next                                            | Up                         |
|---------------------------------------------|-------------------------------------------------|----------------------------|
| [Node customization](node_customization.md) | [Use-case application](use_case_application.md) | [Tutorial root](README.md) |

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
