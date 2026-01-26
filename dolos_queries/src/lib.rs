mod queries;

use clap::{Args, Subcommand};
use gpc_wallet::context::Context;
use queries::{permissioned_candidates, utxos_by_address, utxos_by_address_asset};

const DOLOS_ENDPOINT: &str = "http://localhost:3000";

#[derive(Clone, Debug, Subcommand)]
pub enum DolosQueryCommand {
    #[command(subcommand)]
    DolosQuery(Command),
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Get UTxOs by address
    UtxosByAddress(UtxosByAddressArgs),

    /// Get UTxOs by address filtered by asset
    UtxosByAddressAsset(UtxosByAddressAssetArgs),

    /// Get permissioned candidates from authorities datum
    PermissionedCandidates(PermissionedCandidatesArgs),
}

impl DolosQueryCommand {
    pub async fn run(&self) -> sc_cli::Result<()> {
        let Context { cli, .. } = Context::<DolosQueryCommand>::load_context().await.unwrap();
        match cli.command {
            Some(DolosQueryCommand::DolosQuery(cmd)) => match cmd {
                Command::UtxosByAddress(args) => {
                    let _ = utxos_by_address(args).await.unwrap();
                    Ok(())
                }
                Command::UtxosByAddressAsset(args) => {
                    let _ = utxos_by_address_asset(args).await.unwrap();
                    Ok(())
                }
                Command::PermissionedCandidates(args) => {
                    let _ = permissioned_candidates(args).await.unwrap();
                    Ok(())
                }
            },
            None => {
                log::info!(" Dolos Query Command");
                Ok(())
            }
        }
    }
}

#[derive(Debug, Args, Clone)]
pub struct UtxosByAddressArgs {
    /// The address to query for UTxOs.
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "ADDRESS"
    )]
    pub address: String,
}

#[derive(Debug, Args, Clone)]
pub struct UtxosByAddressAssetArgs {
    /// The address to query for UTxOs.
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "ADDRESS"
    )]
    pub address: String,

    /// The asset to filter UTxOs by.
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "ASSET"
    )]
    pub asset: String,
}

pub type PermissionedCandidatesArgs = UtxosByAddressArgs;
