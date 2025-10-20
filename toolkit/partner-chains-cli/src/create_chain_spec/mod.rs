use crate::config::ConfigFieldDefinition;
use crate::io::IOContext;
use crate::permissioned_candidates::{ParsedPermissionedCandidatesKeys, PermissionedCandidateKeys};
use crate::runtime_bindings::PartnerChainRuntime;
use crate::{config::config_fields, CmdRun};
use anyhow::anyhow;
use authority_selection_inherents::MaybeFromCandidateKeys;
use griffin_core::genesis::config_builder::GenesisConfig;
use partner_chains_plutus_data::permissioned_candidates::permissioned_candidates_to_plutus_data;
use sidechain_domain::{AssetName, MainchainAddress, PolicyId, UtxoId};
use sp_runtime::DeserializeOwned;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, clap::Parser)]
pub struct CreateChainSpecCmd<T: PartnerChainRuntime> {
    #[clap(skip)]
    _phantom: PhantomData<T>,
}

impl<T: PartnerChainRuntime> CmdRun for CreateChainSpecCmd<T> {
    fn run<C: IOContext>(&self, context: &C) -> anyhow::Result<()> {
        let config = CreateChainSpecConfig::load(context)?;
        context.print("This wizard will create a genesis.json to use as chain spec, using the candidates found in the the provided configuration");
        context.print(
                "If the chain includes registered candidates, you need to obtain their keys and add them to the permissioned candidates list in the configuration as well, to set up the genesis accordingly. You need to have all the candidate's keys before moving on, or else they won't be able to participate in the chain.",
            );
        Self::print_config(context, &config);
        if context.prompt_yes_no("Do you want to continue?", true) {
            let initial_permissioned_candidates_data: &Vec<
                sidechain_domain::PermissionedCandidateData,
            > = &config
                .initial_permissioned_candidates_parsed
                .iter()
                .map(From::from)
                .collect::<Vec<sidechain_domain::PermissionedCandidateData>>();
            let encoded_authorities =
                permissioned_candidates_to_plutus_data(initial_permissioned_candidates_data)
                    .to_hex();
            let mut genesis: GenesisConfig = serde_json::from_str(GENESIS_DEFAULT_JSON)?;
            genesis.outputs[0].datum = Some(encoded_authorities);
            context.write_file(
                "new-genesis.json",
                serde_json::to_string_pretty(&genesis)?.as_str(),
            );
            context.print("genesis.json file has been created.");
            context.print(format!("Committee candidates will be found at {}, with the corresponding Authorities token", genesis.outputs[0].address).as_str());
            context
                .print("The rest of the UTxOs can be modified to have the genesis set you need.");
            context.print(
                "If you are the governance authority, you can distribute it to the validators.",
            );
            Ok(())
        } else {
            context.print("Aborted.");
            Ok(())
        }
    }
}

impl<T: PartnerChainRuntime> CreateChainSpecCmd<T> {
    fn print_config<C: IOContext>(context: &C, config: &CreateChainSpecConfig<T::Keys>) {
        context.print("Chain parameters:");
        context.print(format!("- Genesis UTXO: {}", config.genesis_utxo).as_str());
        use colored::Colorize;
        if config.initial_permissioned_candidates_parsed.is_empty() {
            context.print("WARNING: The list of candidates is empty. Generated chain spec will not allow the chain to start.".red().to_string().as_str());
            let update_msg = format!(
				"Update 'initial_permissioned_candidates' field of {} file with keys of the committee.",
				context
					.config_file_path(config_fields::INITIAL_PERMISSIONED_CANDIDATES.config_file)
			);
            context.print(update_msg.red().to_string().as_str());
            context.print(
                INITIAL_PERMISSIONED_CANDIDATES_EXAMPLE
                    .yellow()
                    .to_string()
                    .as_str(),
            );
        } else {
            context.print("Candidates:");
            for candidate in config.initial_permissioned_candidates_raw.iter() {
                context.print(format!("- {}", candidate).as_str());
            }
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
/// Configuration that contains all Partner Chain specific data required to create the chain spec
pub struct CreateChainSpecConfig<Keys> {
    pub genesis_utxo: UtxoId,
    pub initial_permissioned_candidates_raw: Vec<PermissionedCandidateKeys>,
    pub initial_permissioned_candidates_parsed: Vec<ParsedPermissionedCandidatesKeys<Keys>>,
    pub committee_candidate_address: MainchainAddress,
    pub d_parameter_policy_id: PolicyId,
    pub permissioned_candidates_policy_id: PolicyId,
    pub bridge_token_policy: PolicyId,
    pub bridge_token_asset_name: AssetName,
    pub illiquid_circulation_supply_validator_address: MainchainAddress,
    pub governed_map_validator_address: Option<MainchainAddress>,
    pub governed_map_asset_policy_id: Option<PolicyId>,
}

impl<Keys: MaybeFromCandidateKeys> CreateChainSpecConfig<Keys> {
    pub(crate) fn load<C: IOContext>(c: &C) -> Result<Self, anyhow::Error> {
        let initial_permissioned_candidates_raw =
            load_config_field(c, &config_fields::INITIAL_PERMISSIONED_CANDIDATES)?;
        let initial_permissioned_candidates_parsed: Vec<ParsedPermissionedCandidatesKeys<Keys>> =
            initial_permissioned_candidates_raw
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<ParsedPermissionedCandidatesKeys<Keys>>, anyhow::Error>>()?;
        Ok(Self {
            genesis_utxo: load_config_field(c, &config_fields::GENESIS_UTXO)?,
            initial_permissioned_candidates_raw,
            initial_permissioned_candidates_parsed,
            committee_candidate_address: load_config_field(
                c,
                &config_fields::COMMITTEE_CANDIDATES_ADDRESS,
            )?,
            d_parameter_policy_id: load_config_field(c, &config_fields::D_PARAMETER_POLICY_ID)?,
            permissioned_candidates_policy_id: load_config_field(
                c,
                &config_fields::PERMISSIONED_CANDIDATES_POLICY_ID,
            )?,
            bridge_token_policy: load_config_field(c, &config_fields::BRIDGE_TOKEN_POLICY)?,
            bridge_token_asset_name: load_config_field(c, &config_fields::BRIDGE_TOKEN_ASSET_NAME)?,
            illiquid_circulation_supply_validator_address: load_config_field(
                c,
                &config_fields::ILLIQUID_SUPPLY_ADDRESS,
            )?,
            governed_map_validator_address: config_fields::GOVERNED_MAP_VALIDATOR_ADDRESS
                .load_from_file(c),
            governed_map_asset_policy_id: config_fields::GOVERNED_MAP_POLICY_ID.load_from_file(c),
        })
    }
}

impl<T> Default for CreateChainSpecConfig<T> {
    fn default() -> Self {
        Self {
            genesis_utxo: Default::default(),
            initial_permissioned_candidates_raw: Default::default(),
            initial_permissioned_candidates_parsed: Default::default(),
            committee_candidate_address: Default::default(),
            d_parameter_policy_id: Default::default(),
            permissioned_candidates_policy_id: Default::default(),
            bridge_token_policy: Default::default(),
            bridge_token_asset_name: Default::default(),
            illiquid_circulation_supply_validator_address: Default::default(),
            governed_map_validator_address: Default::default(),
            governed_map_asset_policy_id: Default::default(),
        }
    }
}

fn load_config_field<C: IOContext, T: DeserializeOwned>(
    context: &C,
    field: &ConfigFieldDefinition<T>,
) -> Result<T, anyhow::Error> {
    field.load_from_file(context).ok_or_else(|| {
		context.eprint(format!("The '{}' configuration file is missing or invalid.\nIf you are the governance authority, please make sure you have run the `prepare-configuration` command to generate the chain configuration file.\nIf you are a validator, you can obtain the chain configuration file from the governance authority.", context.config_file_path(field.config_file)).as_str());
		anyhow!("failed to read '{}'", field.path.join("."))
	})
}

pub const INITIAL_PERMISSIONED_CANDIDATES_EXAMPLE: &str = r#"Example of 'initial_permissioned_candidates' field with 2 permissioned candidates:
"initial_permissioned_candidates": [
	{
		"partner_chains_key": "0x020a1091341fe5664bfa1782d5e0477968906ac916b04cb365ec3153755684d9a1",
		"keys": {
			"aura": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde49a5684e7a56da27d",
			"gran": "0x88dc3417d5058ec4b4503e0c12ea1a0a89be200f498922423d4334014fa6b0ee"
		}
	},
	{
		"partner_chains_key": "0x0390084fdbf27d2b79d26a4f13f0cdd982cb755a661969143c37cbc49ef5b91f27",
		"keys": {
			"aura": "0x8eaf04151687736326c9fea17e25fc5287613698c912909cb226aa4794f26a48",
			"gran": "0xd17c2d7823ebf260fd138f2d7e27d114cb145d968b5ff5006125f2414fadae69"
		}
	}
]"#;

pub const GENESIS_DEFAULT_JSON: &str = r#"
{
    "zero_time": 1747081100000,
    "zero_slot": 0,
    "outputs": [
        {
            "address": "0000000000000000000000000000000000000000000000000000000000",
            "coin": 314150000,
            "value": [
                    {
                        "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                        "assets": [ ["Authorities", 300000000]]
                    }
                    ],
            "datum": ""
        },
        {
            "address": "6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4",
            "coin": 314000000,
            "value": [
                    {
                        "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                        "assets": [ ["tokenA", 271000000], ["tokenB", 1123581321] ]
                    }
                    ],
            "datum": "820080"
        },
        {
            "address": "61547932e40a24e2b7deb41f31af21ed57acd125f4ed8a72b626b3d7f6",
            "coin": 314150000,
            "value": [
                    {
                        "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                        "assets": [ ["tokenA", 300000000], ["tokenB", 2000000000] ]
                    }
                    ],
            "datum": "820080"
        }
    ]
}
"#;
