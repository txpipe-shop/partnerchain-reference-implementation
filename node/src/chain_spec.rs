use griffin_partner_chains_runtime::WASM_BINARY;
use sc_service::{ChainType, Properties};

/// This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

fn props() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenDecimals".to_string(), 0.into());
    properties.insert("tokenSymbol".to_string(), "MINI".into());
    properties
}

pub fn development_chain_spec() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.expect("Development wasm not available"),
        Default::default(),
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .with_properties(props())
    .build())
}
