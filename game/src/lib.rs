mod game;

use clap::{Subcommand, Args};
use griffin_core::types::Input;
use griffin_wallet::{context::Context, keystore, utils};
use sp_core::H256;

#[derive(Debug, Subcommand)]
pub enum GameCommand {
    /// Create a ship to play the Asteria game
    CreateShip(CreateShipArgs),
}

impl GameCommand {
    pub async fn run(&self) -> sc_cli::Result<()> {
        let Context {cli, client, db, keystore, .. } = Context::<GameCommand>::load_context().await.unwrap();
        match cli.command {
            Some(GameCommand::CreateShip(args)) => {
                let _ = game::create_ship(&db, &client, &keystore, args).await;
                Ok(())
            }
            None => {
                log::info!("No Wallet Command invoked. Exiting.");
                Ok(())
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct CreateShipArgs {
    /// An input to be consumed by this transaction. This argument may be specified multiple times.
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::input_from_string, required = true, value_name = "WALLET_OUTPUT_REF")]
    pub input: Input,

    /// 32-byte H256 public key of an input owner.
    /// Their pk/sk pair must be registered in the wallet's keystore.
    #[arg(long, verbatim_doc_comment, value_parser = utils::h256_from_string, default_value = keystore::SHAWN_PUB_KEY, value_name = "PUBLIC_KEY")]
    pub witness: H256,

    #[arg(
        long,
        verbatim_doc_comment,
        required = true,
        value_name = "POS_X"
    )]
    pub pos_x: i16,

    #[arg(
        long,
        verbatim_doc_comment,
        required = true,
        value_name = "POS_Y"
    )]
    pub pos_y: i16,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "TIME_TO_LIVE"
    )]
    pub ttl: u64,
}