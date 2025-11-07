use griffin_partner_chains_runtime::opaque::SessionKeys;
use partner_chains_cli::{KeyDefinition, AURA, GRANDPA};
use partner_chains_node_commands::{PartnerChainRuntime, PartnerChainsSubcommand};
use game::GameCommand;

#[derive(Debug, Clone)]
pub enum Consensus {
    ManualSeal(u64),
    InstantSeal,
    None,
}

#[derive(Debug, Clone)]
pub struct WizardBindings;

impl PartnerChainRuntime for WizardBindings {
    type Keys = SessionKeys;

    fn key_definitions() -> Vec<KeyDefinition<'static>> {
        vec![AURA, GRANDPA]
    }

    // This function is required by the PartnerChainsRuntime trait
    // Leaving it empty won't work as it parses for the ChainSpec structure whichever it might be
    // We give an implementation using the default genesis
    fn create_chain_spec(
        _config: &partner_chains_cli::CreateChainSpecConfig<SessionKeys>,
    ) -> serde_json::Value {
        let genesis_default: &str = r#"
            {
                "zero_time": 1747081100000,
                "zero_slot": 0,
                "outputs": [
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
                    },
                    {
                        "address": "0000000000000000000000000000000000000000000000000000000000",
                        "coin": 314150000,
                        "value": [
                                {
                                    "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                                    "assets": [ ["Authorities", 300000000]]
                                }
                                ],
                        "datum": "9FD879809F9F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC5820D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D582088DC3417D5058EC4B4503E0C12EA1A0A89BE200FE98922423D4334014FA6B0EEFFFF00FF"
                    }
                ]
            }
            "#;
        serde_json::from_str(genesis_default).unwrap()
    }
}

impl std::str::FromStr for Consensus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "instant-seal" {
            Consensus::InstantSeal
        } else if let Some(block_time) = s.strip_prefix("manual-seal-") {
            Consensus::ManualSeal(block_time.parse().map_err(|_| "invalid block time")?)
        } else if s.to_lowercase() == "none" {
            Consensus::None
        } else {
            return Err("incorrect consensus identifier".into());
        })
    }
}

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(long, default_value = "manual-seal-3000")]
    pub consensus: Consensus,

    #[clap(flatten)]
    pub run: sc_cli::RunCmd,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management cli utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    #[clap(flatten)]
    Game(GameCommand),

    #[clap(flatten)]
    PartnerChains(PartnerChainsSubcommand<WizardBindings>),

    /// Build a chain specification.
    /// DEPRECATED: `build-spec` command will be removed after 1/04/2026. Use `export-chain-spec`
    /// command instead.
    #[deprecated(
        note = "build-spec command will be removed after 1/04/2026. Use export-chain-spec command instead"
    )]
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Export the chain specification.
    ExportChainSpec(sc_cli::ExportChainSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Db meta columns information.
    ChainInfo(sc_cli::ChainInfoCmd),
}
