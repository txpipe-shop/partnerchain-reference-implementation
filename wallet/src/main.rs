//! CLI wallet to demostrate spending and minting transactions.
//!
//! ## Basic usage
//!
//! In terminal, run the node in development mode:
//!
//! ```bash
//! ./target/release/griffin-solochain-node --dev
//! ```
//!
//! In another terminal, one can interact with the node by issuing wallet
//! commands. Every time the wallet starts (without the `--help` or `--version`
//! command-line options), it will try to synchronize its database with the
//! present chain state through RPC port 9944 (the [DEFAULT_ENDPOINT]), unless
//! there is a mismatch with the genesis hash.
//!
//! To list the whole UTxO set, run
//!
//! ```bash
//! ./target/release/griffin-wallet show-all-outputs
//! ```

extern crate alloc;

use alloc::{string::String};
use griffin_core::{
    pallas_crypto::hash::Hasher as PallasHasher,
    types::Value,
};
use hex::FromHex;
use parity_scale_codec::{Encode};

mod cli;
mod command;
mod keystore;
mod order_book;
mod rpc;
mod sync;
mod context;
mod utils;

use cli::{Command};
use context::{Context};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Context {cli, client, db, keystore, data_path, keystore_path } = Context::<Command>::load_context().await.unwrap();
    // Dispatch to proper subcommand
    match cli.command {
        Some(Command::VerifyUtxo { input }) => {
            println!("Details of coin {}:", hex::encode(input.encode()));

            // Print the details from storage
            let coin_from_storage = utils::get_coin_from_storage(&input, &client).await?;
            print!("Found in storage.  Value: {:?}, ", coin_from_storage);

            // Print the details from the local db
            match sync::get_unspent(&db, &input)? {
                Some((owner, amount, _)) => {
                    println!("Found in local db. Value: {amount:?}, owned by {owner}");
                }
                None => {
                    println!("Not found in local db");
                }
            }

            Ok(())
        }
        Some(cli::Command::SpendValue(args)) => {
            command::spend_value(&db, &client, &keystore, args).await
        }
        Some(Command::InsertKey { seed }) => keystore::insert_key(&keystore, &seed),
        Some(Command::GenerateKey { password }) => {
            keystore::generate_key(&keystore, password)?;
            Ok(())
        }
        Some(Command::ShowKeys) => {
            keystore::get_keys(&keystore)?.for_each(|pubkey| {
                let pk_str: &str = &hex::encode(pubkey);
                let hash: String =
                    PallasHasher::<224>::hash(&<[u8; 32]>::from_hex(pk_str).unwrap()).to_string();
                println!("key: 0x{}; addr: 0x61{}", pk_str, hash);
            });

            Ok(())
        }
        Some(Command::RemoveKey { pub_key }) => {
            println!(
                "CAUTION!!! About permanently remove {pub_key}. This action CANNOT BE REVERSED. Type \"proceed\" to confirm deletion."
            );

            let mut confirmation = String::new();
            std::io::stdin()
                .read_line(&mut confirmation)
                .expect("Failed to read line");

            if confirmation.trim() == "proceed" {
                keystore::remove_key(&keystore_path, &pub_key)
            } else {
                println!("Deletion aborted. That was close.");
                Ok(())
            }
        }
        Some(Command::ShowBalance) => {
            println!("Balance Summary");
            let mut total = Value::Coin(0);
            let balances = sync::get_balances(&db)?;
            for (account, balance) in balances {
                total += balance.clone();
                println!("{account}: {balance}");
            }
            println!("{:-<58}", "");
            println!("Total:   {}", total.normalize());

            Ok(())
        }
        Some(Command::ShowAllOutputs) => {
            println!("###### Unspent outputs ###########");
            sync::show_outputs(sync::print_unspent_tree(&db)?);
            println!("To see all details of a particular UTxO, invoke the `verify-utxo` command.");
            Ok(())
        }
        Some(Command::ShowOutputsAt(args)) => {
            println!(
                "###### Unspent outputs at address {} ###########",
                args.address
            );
            sync::show_outputs(sync::get_outputs_at(&db, args)?);
            println!("To see all details of a particular UTxO, invoke the `verify-utxo` command.");
            Ok(())
        }
        Some(Command::ShowOutputsWithAsset(args)) => {
            println!(
                "###### Unspent outputs containing asset with name {} and policy ID {} ###########",
                args.name, args.policy
            );
            sync::show_outputs(sync::get_outputs_with_asset(&db, args)?);
            println!("To see all details of a particular UTxO, invoke the `verify-utxo` command.");
            Ok(())
        }
        Some(Command::ShowAllOrders) => {
            println!("###### Available Orders ###########");
            sync::print_orders(&db)?;
            Ok(())
        }
        Some(cli::Command::BuildTx(args)) => command::build_tx(&db, &client, &keystore, args).await,
        None => {
            log::info!("No Wallet Command invoked. Exiting.");
            Ok(())
        }
    }?;

    if cli.tmp || cli.dev {
        // Cleanup the temporary directory.
        std::fs::remove_dir_all(data_path.clone()).map_err(|e| {
            log::warn!(
                "Unable to remove temporary data directory at {}\nPlease remove it manually.",
                data_path.to_string_lossy()
            );
            e
        })?;
    }

    Ok(())
}
