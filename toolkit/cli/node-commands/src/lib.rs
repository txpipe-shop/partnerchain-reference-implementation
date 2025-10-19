//! This crate provides an enum type [PartnerChainsSubcommand] collecting all Partner Chains specific subcommands,
//! and a [run] function for running these commands.
//! [PartnerChainsSubcommand] is meant to be used by a command line argument parser library.
#![deny(missing_docs)]
use clap::Parser;
use cli_commands::registration_signatures::RegistrationSignaturesCmd;
use frame_support::sp_runtime::traits::NumberFor;
use partner_chains_cli::DefaultCmdRunContext;
pub use partner_chains_cli::PartnerChainRuntime;
use partner_chains_smart_contracts_commands::SmartContractsCmd;
use sc_cli::{CliConfiguration, SharedParams};
use sidechain_domain::*;
use sp_runtime::traits::Block as BlockT;

#[derive(Debug, Clone, Parser)]
/// Command line arguments for the `ariadne-parameters` command.
pub struct AriadneParametersCmd {
    #[arg(long)]
    /// Main chain epoch number for which the parameters should be queried.
    pub mc_epoch_number: McEpochNumber,
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub shared_params: SharedParams,
}

impl CliConfiguration for AriadneParametersCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.shared_params
    }
}

#[derive(Debug, Clone, Parser)]
/// Command line arguments for the `sidechain-params` command.
pub struct SidechainParamsCmd {
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub shared_params: SharedParams,
}

impl CliConfiguration for SidechainParamsCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.shared_params
    }
}

#[derive(Debug, Clone, Parser)]
/// Command line arguments for the `registration-status` command.
pub struct RegistrationStatusCmd {
    #[arg(long)]
    #[arg(long, alias = "mainchain-pub-key")]
    /// Stake pool public key for which the registration status should be returned.
    pub stake_pool_pub_key: StakePoolPublicKey,
    #[arg(long)]
    /// Mainchain epoch number for which the registration status should be returned.
    pub mc_epoch_number: McEpochNumber,
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub shared_params: SharedParams,
}

impl CliConfiguration for RegistrationStatusCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.shared_params
    }
}

#[derive(Clone, Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
/// Entry point for all Partner Chains specific subcommand.
pub enum PartnerChainsSubcommand<RuntimeBindings: PartnerChainRuntime + Send + Sync> {
    /// Generates registration signatures for partner chains committee candidates
    RegistrationSignatures(RegistrationSignaturesCmd),

    /// Commands for interacting with Partner Chain smart contracts on Cardano
    #[command(subcommand)]
    SmartContracts(SmartContractsCmd),

    /// Partner Chains text "wizards" for setting up chain
    #[command(subcommand)]
    Wizards(partner_chains_cli::Command<RuntimeBindings>),
}

#[allow(deprecated)]
/// Runs a Partner Chains subcommand.
pub fn run<Block, RuntimeBindings: PartnerChainRuntime + Send + Sync>(
    cmd: PartnerChainsSubcommand<RuntimeBindings>, // partnerchainAddress
) -> sc_cli::Result<()>
where
    Block: BlockT,
    NumberFor<Block>: From<u32> + Into<u32>,
{
    match cmd {
        PartnerChainsSubcommand::RegistrationSignatures(cmd) => Ok(println!("{}", cmd.execute())),
        PartnerChainsSubcommand::SmartContracts(cmd) => {
            setup_log4rs()?;
            Ok(cmd.execute_blocking()?)
        }
        PartnerChainsSubcommand::Wizards(cmd) => {
            setup_log4rs()?;
            Ok(cmd
                .run(&DefaultCmdRunContext)
                .map_err(|e| sc_cli::Error::Application(e.into()))?)
        }
    }
}

/// This sets logging to stderr, leaving stdout for smart-contracts JSON outputs.
/// Ogmios interactions are logged to a file.
fn setup_log4rs() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stderr = log4rs::append::console::ConsoleAppender::builder()
        .target(log4rs::append::console::Target::Stderr)
        .build();
    let ogmios_log = log4rs::append::file::FileAppender::builder().build("ogmios_client.log")?;

    let log_config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("stderr", Box::new(stderr)))
        .appender(log4rs::config::Appender::builder().build("ogmios-log", Box::new(ogmios_log)))
        .logger(
            log4rs::config::Logger::builder()
                .appender("ogmios-log")
                .additive(false)
                .build("ogmios_client::jsonrpsee", log::LevelFilter::Debug),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("stderr")
                .build(log::LevelFilter::Info),
        )?;

    log4rs::init_config(log_config)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    async fn some_err() -> Result<String, String> {
        Err("some err".to_string())
    }

    #[tokio::test]
    async fn print_async_doesnt_fail_if_result_is_error() {
        let result = super::print_result(some_err()).await;
        assert!(result.is_ok());
    }
}
