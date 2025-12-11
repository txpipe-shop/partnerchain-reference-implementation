mod game;
mod tests;

use clap::{Args, Subcommand};
use griffin_core::types::Input;
use gpc_wallet::{context::Context, keystore, utils};
use sp_core::H256;

#[derive(Clone, Debug, Subcommand)]
pub enum GameCommand {
    #[command(subcommand)]
    Game(Command),
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Create a ship to enter the game
    CreateShip(CreateShipArgs),
    /// Gather fuel using a ship and a fuel pellet
    GatherFuel(GatherFuelArgs),
    /// Move a ship to a new position
    MoveShip(MoveShipArgs),
    /// Mine Asteria using a ship
    MineAsteria(MineAsteriaArgs),
    /// Apply parameters and write game scripts
    DeployScripts(DeployScriptsArgs),
}

impl GameCommand {
    pub async fn run(&self) -> sc_cli::Result<()> {
        let Context {
            cli,
            client,
            db,
            keystore,
            slot_config,
            ..
        } = Context::<GameCommand>::load_context().await.unwrap();
        match cli.command {
            Some(GameCommand::Game(cmd)) => match cmd {
                Command::CreateShip(args) => {
                    let _ = game::create_ship(&db, &client, &keystore, slot_config, args).await;
                    Ok(())
                }
                Command::GatherFuel(args) => {
                    let _ = game::gather_fuel(&db, &client, &keystore, args).await;
                    Ok(())
                }
                Command::MoveShip(args) => {
                    let _ = game::move_ship(&db, &client, &keystore, slot_config, args).await;
                    Ok(())
                }
                Command::MineAsteria(args) => {
                    let _ = game::mine_asteria(&db, &client, &keystore, args).await;
                    Ok(())
                }
                Command::DeployScripts(args) => {
                    let _ = game::deploy_scripts(args).await;
                    Ok(())
                }
            },
            None => {
                log::info!(" Asteria game");
                Ok(())
            }
        }
    }
}

#[derive(Debug, Args, Clone)]
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
        allow_negative_numbers = true,
        value_name = "POS_X"
    )]
    pub pos_x: i16,

    #[arg(
        long,
        verbatim_doc_comment,
        required = true,
        allow_negative_numbers = true,
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

    /// Path to the game parameters JSON file
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "GAME_PARAMS_PATH"
    )]
    pub params_path: String,
}

#[derive(Debug, Args, Clone)]
pub struct GatherFuelArgs {
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::input_from_string, required = true, value_name = "SHIP_OUTPUT_REF")]
    pub ship: Input,

    #[arg(long, short, verbatim_doc_comment, value_parser = utils::input_from_string, required = true, value_name = "PELLET_OUTPUT_REF")]
    pub pellet: Input,

    /// 32-byte H256 public key of an input owner.
    /// Their pk/sk pair must be registered in the wallet's keystore.
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::h256_from_string, default_value = keystore::SHAWN_PUB_KEY, value_name = "PUBLIC_KEY")]
    pub witness: H256,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "FUEL_AMOUNT"
    )]
    pub fuel: u64,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "VALIDITY_INTERVAL_START"
    )]
    pub validity_interval_start: u64,

    /// Path to the game parameters JSON file
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "GAME_PARAMS_PATH"
    )]
    pub params_path: String,
}

#[derive(Debug, Args, Clone)]
pub struct MoveShipArgs {
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::input_from_string, required = true, value_name = "SHIP_OUTPUT_REF")]
    pub ship: Input,

    /// 32-byte H256 public key of an input owner.
    /// Their pk/sk pair must be registered in the wallet's keystore.
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::h256_from_string, default_value = keystore::SHAWN_PUB_KEY, value_name = "PUBLIC_KEY")]
    pub witness: H256,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        allow_negative_numbers = true,
        value_name = "POS_X"
    )]
    pub pos_x: i16,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        allow_negative_numbers = true,
        value_name = "POS_Y"
    )]
    pub pos_y: i16,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "VALIDITY_INTERVAL_START"
    )]
    pub validity_interval_start: u64,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "TIME_TO_LIVE"
    )]
    pub ttl: u64,

    /// Path to the game parameters JSON file
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "GAME_PARAMS_PATH"
    )]
    pub params_path: String,
}

#[derive(Debug, Args, Clone)]
pub struct MineAsteriaArgs {
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::input_from_string, required = true, value_name = "SHIP_OUTPUT_REF")]
    pub ship: Input,

    /// 32-byte H256 public key of an input owner.
    /// Their pk/sk pair must be registered in the wallet's keystore.
    #[arg(long, short, verbatim_doc_comment, value_parser = utils::h256_from_string, default_value = keystore::SHAWN_PUB_KEY, value_name = "PUBLIC_KEY")]
    pub witness: H256,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "VALIDITY_INTERVAL_START"
    )]
    pub validity_interval_start: u64,

    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "MINE_COIN_AMOUNT"
    )]
    pub mine_coin_amount: u64,

    /// Path to the game parameters JSON file
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "GAME_PARAMS_PATH"
    )]
    pub params_path: String,
}

#[derive(Debug, Args, Clone)]
pub struct DeployScriptsArgs {
    /// Path to the game parameters JSON file
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "GAME_PARAMS_PATH"
    )]
    pub params_path: String,
}
