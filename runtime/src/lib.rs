//! The runtime contains the core logic of the ledger run by the Griffin node.

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

pub mod genesis;

use alloc::{string::ToString, vec, vec::Vec};
use griffin_core::genesis::config_builder::GenesisConfig;
use griffin_core::types::{Address, AssetName, Input, PolicyId};
use griffin_core::utxo_set::TransparentUtxoSet;
use griffin_core::MILLI_SECS_PER_SLOT;
pub use opaque::SessionKeys;

use parity_scale_codec::{Decode, Encode};
use polkadot_sdk_frame::runtime::apis;
use scale_info::TypeInfo;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::OpaqueMetadata;
use sp_inherents::InherentData;
use sp_runtime::{
    impl_opaque_keys,
    traits::Block as BlockT,
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, BoundToRuntimeAppPublic,
};

use demo_authorities as Authorities;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::{runtime_version, RuntimeVersion};

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    use sp_core::{ed25519, sr25519};

    // This part is necessary for generating session keys in the runtime
    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: AuraAppPublic,
            pub grandpa: GrandpaAppPublic,
        }
    }
    impl From<(sr25519::Public, ed25519::Public)> for SessionKeys {
        fn from((aura, grandpa): (sr25519::Public, ed25519::Public)) -> Self {
            Self {
                aura: aura.into(),
                grandpa: grandpa.into(),
            }
        }
    }
    // Typically these are not implemented manually, but rather for the pallet associated with the
    // keys. Here we are not using the pallets, and these implementations are trivial, so we just
    // re-write them.
    pub struct AuraAppPublic;
    impl BoundToRuntimeAppPublic for AuraAppPublic {
        type Public = AuraId;
    }

    pub struct GrandpaAppPublic;
    impl BoundToRuntimeAppPublic for GrandpaAppPublic {
        type Public = sp_consensus_grandpa::AuthorityId;
    }
}

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("griffin-solochain-runtime"),
    impl_name: alloc::borrow::Cow::Borrowed("griffin-solochain-runtime"),
    authoring_version: 1,
    spec_version: 0,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

pub type Transaction = griffin_core::types::Transaction;
pub type Block = griffin_core::types::Block;
pub type Executive = griffin_core::Executive;
pub type Output = griffin_core::types::Output;

/// The main struct in this module.
#[derive(Encode, Decode, PartialEq, Eq, Clone, TypeInfo)]
pub struct Runtime;

impl_runtime_apis! {
    impl griffin_core::utxo_set::TransparentUtxoSetApi<Block> for Runtime {
        fn peek_utxo(input: &Input) -> Option<Output> {
            TransparentUtxoSet::peek_utxo(input)
        }

        fn peek_utxo_by_address(addr: &Address) -> Vec<Output> {
            TransparentUtxoSet::peek_utxos_from_address(addr)
        }

        fn peek_utxo_with_asset(asset_name: &AssetName, asset_policy: &PolicyId) -> Vec<Output> {
            TransparentUtxoSet::peek_utxos_with_asset(asset_name, asset_policy)
        }
    }

    // https://substrate.dev/rustdocs/master/sp_api/trait.Core.html
    impl apis::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::open_block(header)
        }
    }

    impl apis::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Vec::new())
        }

        fn metadata_at_version(_version: u32) -> Option<OpaqueMetadata> {
            None
        }

        fn metadata_versions() -> alloc::vec::Vec<u32> {
            Default::default()
        }
    }

    impl apis::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::close_block()
        }

        fn inherent_extrinsics(_data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            Vec::new()
        }

        fn check_inherents(
            _block: Block,
            _data: InherentData
        ) -> sp_inherents::CheckInherentsResult {
            sp_inherents::CheckInherentsResult::new()
        }
    }

    impl apis::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl apis::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl apis::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(MILLI_SECS_PER_SLOT.into())
        }

        fn authorities() -> Vec<AuraId> {
            Authorities::aura_authorities()
        }
    }

    impl apis::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Authorities::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            0u64
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                sp_runtime::traits::NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: sp_consensus_grandpa::AuthorityId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            None
        }
    }

    impl apis::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            let genesis_config = serde_json::from_slice::<GenesisConfig>(config.as_slice())
                .map_err(|_| "The input JSON is not a valid genesis configuration.")?;

            griffin_core::genesis::GriffinGenesisConfigBuilder::build(genesis_config)
        }

        fn get_preset(_id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            let genesis_config : &GenesisConfig = &genesis::get_genesis_config("".to_string());
            Some(serde_json::to_vec(genesis_config)
                 .expect("Genesis configuration is valid."))
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }
}
