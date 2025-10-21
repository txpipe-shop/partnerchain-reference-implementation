# Dev Activity Log pt. 2 - Partner Chain Integration

## Part 1 - Code Integration

The first step of the process is to include the relevant code  modifications.

The Substrate node uses IOG’s Partner Chain SDK to connect with Cardano. This SDK provides a lot of utilities for the node but most of them are implemented using the account model, so we decided on not using those features. We exclusively use the `partner-chains-cli` which is meant for the creation of the partner chain, the initial set up of candidates, and the registration of candidates. Moreover, we’ve modified some parts to better adjust to our needs.

Let’s get started with the modifications:

1. Add `sidechain-domain` and `utils` from Partner-chains SDK `toolkit`. Add as workspace members and as workspace dependencies. Add additional dependencies.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L211

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L220-L226

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L28-L37

2. Add crosschain Key implementation to runtime:
    1. Add deps to Cargo.toml

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/runtime/Cargo.toml#L31-L33

    2. Import KeyTypeId from sp_core

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/runtime/src/lib.rs#L25

    3. Implement CrossChain App in Opaque module.

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/runtime/src/lib.rs#L80-L123

3. Add `authoritiy_selection_inherents`, `primitives`, `query`, `selection` from `CommitteeSelection`. Add `primitives`, `block-search` and `slots` from `sidechain`.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L203-L213

4. Add `partner-chains-cli`.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L198-L199

    Here we modified some things, namely the removal of references to pallets.

    1. In Cargo.toml: remove pallets as dependencies.

        ```rust
        # pallet-session-validator-management = { workspace = true, features = ["std"] }
        # pallet-partner-chains-bridge = { workspace = true, features = ["std"] }
        # sp-partner-chains-bridge = { workspace = true, features = ["std"] }
        # pallet-partner-chains-session = { workspace = true, features = ["std"] }
        # pallet-sidechain = { workspace = true, features = ["std"] }
        # pallet-governed-map = { workspace = true, features = ["std"] }
        # sp-governed-map = { workspace = true, features = ["std"] }
        ```

    2. In create_chain_spec, remove the following lines:

        ```rust
        use sp_core::ecdsa;
        use sp_runtime::AccountId32;
        // /// Returns [pallet_sidechain::GenesisConfig] derived from the config
        // pub fn pallet_sidechain_config<T: pallet_sidechain::Config>(
        //  &self,
        //  slots_per_epoch: sidechain_slots::SlotsPerEpoch,
        // ) -> pallet_sidechain::GenesisConfig<T> {
        //  pallet_sidechain::GenesisConfig {
        //      genesis_utxo: self.genesis_utxo,
        //      slots_per_epoch,
        //      _config: PhantomData,
        //  }
        // }

        // /// Returns [pallet_partner_chains_session::GenesisConfig] derived from the config, using initial permissioned candidates
        // /// as initial validators
        // pub fn pallet_partner_chains_session_config<T: pallet_partner_chains_session::Config>(
        //  &self,
        // ) -> pallet_partner_chains_session::GenesisConfig<T>
        // where
        //  T::ValidatorId: From<AccountId32>,
        //  T::Keys: From<Keys>,
        // {
        //  pallet_partner_chains_session::GenesisConfig {
        //      initial_validators: self
        //          .initial_permissioned_candidates_parsed
        //          .iter()
        //          .map(|c| (c.account_id_32().into(), c.keys.clone().into()))
        //          .collect::<Vec<_>>(),
        //  }
        // }

        // /// Returns [pallet_session_validator_management::GenesisConfig] derived from the config using initial permissioned candidates
        // /// as initial authorities
        // pub fn pallet_session_validator_management_config<
        //  T: pallet_session_validator_management::Config,
        // >(
        //  &self,
        // ) -> pallet_session_validator_management::GenesisConfig<T>
        // where
        //  T::AuthorityId: From<ecdsa::Public>,
        //  T::AuthorityKeys: From<Keys>,
        //  T::CommitteeMember:
        //      From<authority_selection_inherents::CommitteeMember<T::AuthorityId, T::AuthorityKeys>>,
        // {
        //  pallet_session_validator_management::GenesisConfig {
        //      initial_authorities: self
        //          .initial_permissioned_candidates_parsed
        //          .iter()
        //          .map(|c| {
        //              authority_selection_inherents::CommitteeMember::permissioned(
        //                  c.sidechain.into(),
        //                  c.keys.clone().into(),
        //              )
        //              .into()
        //          })
        //          .collect::<Vec<_>>(),
        //      main_chain_scripts: sp_session_validator_management::MainChainScripts {
        //          committee_candidate_address: self.committee_candidate_address.clone(),
        //          d_parameter_policy_id: self.d_parameter_policy_id.clone(),
        //          permissioned_candidates_policy_id: self.permissioned_candidates_policy_id.clone(),
        //      },
        //  }
        // }

        // /// Returns [pallet_partner_chains_bridge::GenesisConfig] derived from the config
        // pub fn bridge_config<T: pallet_partner_chains_bridge::Config>(
        //  &self,
        // ) -> pallet_partner_chains_bridge::GenesisConfig<T> {
        //  pallet_partner_chains_bridge::GenesisConfig {
        //      main_chain_scripts: Some(sp_partner_chains_bridge::MainChainScripts {
        //          token_policy_id: self.bridge_token_policy.clone(),
        //          token_asset_name: self.bridge_token_asset_name.clone(),
        //          illiquid_circulation_supply_validator_address: self
        //              .illiquid_circulation_supply_validator_address
        //              .clone(),
        //      }),
        //      initial_checkpoint: Some(self.genesis_utxo),
        //      _marker: PhantomData,
        //  }
        // }

        // /// Returns [pallet_governed_map::GenesisConfig] derived from the config
        // pub fn governed_map_config<T: pallet_governed_map::Config>(
        //  &self,
        // ) -> pallet_governed_map::GenesisConfig<T> {
        //  pallet_governed_map::GenesisConfig {
        //      main_chain_scripts: self.governed_map_validator_address.as_ref().and_then(|addr| {
        //          self.governed_map_asset_policy_id.as_ref().map(|policy| {
        //              sp_governed_map::MainChainScriptsV1 {
        //                  validator_address: addr.clone(),
        //                  asset_policy_id: policy.clone(),
        //              }
        //          })
        //      }),
        //      _marker: PhantomData,
        //  }
        // }
        ```

5. Add `offchain`, `commands` and `plutus-data` from `smart-contracts`.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L215-L218

    Here we also needed to remove some things for some functionalities that we won’t support. There are many changes to the original SDK here so if you’re following our pallet-less approach it might be more convenient to just copy from our source code, but the modifications are detailed below anyways:

    1. Remove references and implementations of `from_ogmios_utxo`, `AssetIdExt`, `get_asset_amount` in `offchain/src/csl`.
    2. Remove the following modules and their references in lib.rs:
        1. `versioning_system`.
        2. `bridge`.
        3. `reserve`.

6. Add `node-commands` and `commands` from `cli`.

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/Cargo.toml#L200-L201

    Again some modules were removed as they don’t fit our implementation.

    1. Node commands

        1. Remove imports

            1. `authority-selection-inherent` import
            2. `address_association_signatures` and `block_producer_metadata_signatures` from cli-commands
            3. Decode encode
            4. sc_service::task manager
            5. use sp_api::ProvideRuntimeApi;
            6. use sp_blockchain::HeaderBackend;
            7. use sp_runtime::AccountId32;
            8. use sp_runtime::DeserializeOwned;
            9. use sp_runtime::Serialize;
            10. use sp_session_validator_management::CommitteeMember as CommitteeMemberT;
            11. use sp_session_validator_management::SessionValidatorManagementApi;
            12. use sp_session_validator_management_query::SessionValidatorManagementQuery;
            13. use sp_session_validator_management_query::commands::*;
            14. use sp_sidechain::{GetGenesisUtxo, GetSidechainStatus};
            15. use std::sync::Arc;
            16. Future
            17. SubstrateCli from sc_cli;

        2. In PartnerChains SubCommand remove the following:

            1. PartnerChainsAddress type parameter
            2. SidechainParams
            3. RegistrationStatus
            4. AriadneParameters
            5. SignAddressAssociation
            6. SignBlockProducerMetadata

        3. Remove REGISTRATION_STATUS_AFTER_HELP.

        4. In run:

            1. Remove Cli, CommitteeMember, Client, BlockProducerMetadata,PartnerchainAddress type parameters, and its respective constraints on the function.
            2. Remove `cli` and `get_deps` function parameters.
            3. Remove `SidechainParams`, `RegistrationStatus`, `AriadneParameters`, `SignAddressAssociation` and `SignBlockProducerMetadata` command
            4. Remove `print_Result` (optional but it will cause an _unused_ warning)

    2. `commands`:

        1. `address_association_signatures`, `get_genesis_utxo` and `block_producer_metadata_signatures` mod inclusions and their respective files.

7. Add partner-chains command to the node.

    1. Add dependencies to the node:

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/node/Cargo.toml#L44-L45

    2. In cli:

        1. Imports

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/node/src/cli.rs#L1-L3

        2. Define PartnerChainRuntime’s WizardBindings:

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/node/src/cli.rs#L12-L13

        3. Add PartnerChains command

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/node/src/cli.rs#L101-L109

    3. In command:

        1. In run function add parsing and implementation for PartnerChain command.

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/node/src/command.rs#L47-L63
        
8. Add From SessionKeys for MaybeAuthorities

    1. Add authority-selection-inherents dependency on runtime.

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/runtime/Cargo.toml#L34

        add also as STD Feature:

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/runtime/Cargo.toml#L39-L42

    2. Add MaybeFromCandidateKeys for SessionKeys in opaque.

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/runtime/src/lib.rs#L124-L126

9. Create-chain-spec: The command was modified to generate the genesis.json file that we need for Griffin. 
    1. Imports in Cargo.toml:

        https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/toolkit/partner-chains-cli/Cargo.toml#L44-L45

    2. In create_chain_spec/mod:
        1. Add imports

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs#L8-L9

        2. Modify main function.
        3. Add constant default genesis to build from.

            https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/c4bf41c11ab089a7b27a4cc0f82511d84dfc731d/toolkit/partner-chains-cli/src/create_chain_spec/mod.rs#L189-L229
