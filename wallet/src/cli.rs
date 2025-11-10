//! Test Wallet's Command Line Interface.

extern crate alloc;

use std::path::PathBuf;

use crate::{
    utils::{address_from_string, h224_from_string, h256_from_string, input_from_string},
    keystore::{SHAWN_ADDRESS, SHAWN_PUB_KEY},
    context::{DEFAULT_ENDPOINT, Context},
    command, sync, keystore, utils
};
use alloc::{string::String};
use clap::{ArgAction::Append, Args, Parser, Subcommand};
use griffin_core::{
    pallas_crypto::hash::Hasher as PallasHasher,
    types::{Address, Coin, Input, Value, PolicyId},
};
use hex::FromHex;
use parity_scale_codec::{Encode};
use sp_core::H256;

/// The wallet's main CLI struct
#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli<T: clap::Subcommand + clap::FromArgMatches> {
    #[arg(long, short, default_value_t = DEFAULT_ENDPOINT.to_string())]
    /// RPC endpoint of the node that this wallet will connect to.
    pub endpoint: String,

    #[arg(long, short('d'))]
    /// Path where the wallet data is stored. Default value is platform specific.
    pub base_path: Option<PathBuf>,

    #[arg(long, verbatim_doc_comment)]
    /// Skip the initial sync that the wallet typically performs with the node.
    /// The wallet will use the latest data it had previously synced.
    pub no_sync: bool,

    #[arg(long)]
    /// A temporary directory will be created to store the configuration and will be deleted at the end of the process.
    /// path will be ignored if this is set.
    pub tmp: bool,

    #[arg(long, verbatim_doc_comment)]
    /// Specify a development wallet instance, using a temporary directory (like --tmp).
    /// The keystore will contain the development key Shawn.
    pub dev: bool,

    #[arg(long, verbatim_doc_comment)]
    /// Erases the wallet DB before starting.
    pub purge_db: bool,

    #[command(subcommand)]
    pub command: Option<T>,
}

/// The tasks supported by the wallet
#[derive(Clone, Debug, Subcommand)]
pub enum WalletCommand {
    #[command(subcommand)]
    Wallet(Command)
}

impl WalletCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        let Context {cli, client, db, keystore, data_path, keystore_path } = Context::<WalletCommand>::load_context().await.unwrap();
        // Dispatch to proper subcommand
        match cli.command {
            Some(WalletCommand::Wallet(cmd)) => 
                match cmd {
                    Command::VerifyUtxo { input } => {
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
                    Command::SpendValue(args) => {
                        command::spend_value(&db, &client, &keystore, args).await
                    }
                    Command::InsertKey { seed } => keystore::insert_key(&keystore, &seed),
                    Command::GenerateKey { password } => {
                        keystore::generate_key(&keystore, password)?;
                        Ok(())
                    }
                    Command::ShowKeys => {
                        keystore::get_keys(&keystore)?.for_each(|pubkey| {
                            let pk_str: &str = &hex::encode(pubkey);
                            let hash: String =
                                PallasHasher::<224>::hash(&<[u8; 32]>::from_hex(pk_str).unwrap()).to_string();
                            println!("key: 0x{}; addr: 0x61{}", pk_str, hash);
                        });
        
                        Ok(())
                    }
                    Command::RemoveKey { pub_key } => {
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
                    Command::ShowBalance => {
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
                    Command::ShowAllOutputs => {
                        println!("###### Unspent outputs ###########");
                        sync::show_outputs(sync::print_unspent_tree(&db)?);
                        println!("To see all details of a particular UTxO, invoke the `verify-utxo` command.");
                        Ok(())
                    }
                    Command::ShowOutputsAt(args) => {
                        println!(
                            "###### Unspent outputs at address {} ###########",
                            args.address
                        );
                        sync::show_outputs(sync::get_outputs_at(&db, args)?);
                        println!("To see all details of a particular UTxO, invoke the `verify-utxo` command.");
                        Ok(())
                    }
                    Command::ShowOutputsWithAsset(args) => {
                        println!(
                            "###### Unspent outputs containing asset with name {} and policy ID {} ###########",
                            args.name, args.policy
                        );
                        sync::show_outputs(sync::get_outputs_with_asset(&db, args)?);
                        println!("To see all details of a particular UTxO, invoke the `verify-utxo` command.");
                        Ok(())
                    }
                    Command::ShowAllOrders => {
                        println!("###### Available Orders ###########");
                        sync::print_orders(&db)?;
                        Ok(())
                    },
                    Command::BuildTx(args) => command::build_tx(&db, &client, &keystore, args).await,

                },
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
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Verify that a particular output ref exists.
    /// Show its value and owner address from both chain storage and the local database.
    #[command(verbatim_doc_comment)]
    VerifyUtxo {
        /// A hex-encoded output reference
        #[arg(value_parser = input_from_string)]
        input: Input,
    },

    /// Send `Value`s to a given address.
    #[command(verbatim_doc_comment)]
    SpendValue(SpendValueArgs),

    /// Insert a private key into the keystore to later use when signing transactions.
    InsertKey {
        /// Seed phrase of the key to insert.
        seed: String,
    },

    /// Generate a private key using either some or no password and insert into the keystore.
    GenerateKey {
        /// Initialize a public/private key pair with a password
        password: Option<String>,
    },

    /// Show public information about all the keys in the keystore.
    ShowKeys,

    /// Remove a specific key from the keystore.
    /// WARNING! This will permanently delete the private key information.
    /// Make sure your keys are backed up somewhere safe.
    #[command(verbatim_doc_comment)]
    RemoveKey {
        /// The public key to remove
        #[arg(value_parser = h256_from_string)]
        pub_key: H256,
    },

    /// For each key tracked by the wallet, shows the sum of all UTXO values owned by that key.
    /// This sum is sometimes known as the "balance".
    #[command(verbatim_doc_comment)]
    ShowBalance,

    /// Show the complete list of UTXOs known to the wallet.
    ShowAllOutputs,

    /// Show the list of UTXOs known to the wallet, filtered by a specific address.
    ShowOutputsAt(ShowOutputsAtArgs),

    /// Show the list of UTXOs known to the wallet, filtered by a specific asset.
    ShowOutputsWithAsset(ShowOutputsWithAssetArgs),

    /// Show the list of UTxOs sitting at the order book example script address.
    ShowAllOrders,

    /// Build a complete Griffin transaction from a JSON file containing all the necessary information.
    BuildTx(BuildTxArgs),
}

/// Arguments for building a complete Griffin transaction.
#[derive(Clone, Debug, Args)]
pub struct BuildTxArgs {
    /// Path to the file containing all the transaction information in JSON format.
    /// There are example contracts and json files for testing this command in the `eutxo_examples` directory.
    /// The file must contain the following fields:
    /// - `inputs_info`: A list of input information objects.
    ///    Each input info contains the following fields:
    ///     - `tx_hash`: The hash of the transaction containing the output to be used as input.
    ///     - `index`: The index of the output in the transaction.
    ///     - `redeemer_cbor`: The cbor-encoded redeemer (optional, for script inputs).
    /// - `outputs_info`: A list of output information objects.
    ///    Each output info contains the following fields:
    ///     - `address`: The address of the output.
    ///     - `coin`: An amount of `Coin`s to be included in the output.
    ///     - `value`: A list of asset bundles to be included in the output.
    ///       Each asset bundle contains the following fields:
    ///         - `policy`: The policy ID of the asset bundle.
    ///         - `assets`: A list of tuples containing the asset name and the amount to be included.
    ///     - `datum`: The hex-encoded datum (optional, for script outputs).
    /// - `scripts_info`: A list of JSON objects containing the hex of plutus scripts
    ///    and their parameters (if any) to be applied to the scripts.
    ///    Each object must contain the following fields:
    ///     - `script_hex`: The hex-encoded script.
    ///     - `script_params_cbor`: The cbor-encoded parameter list (optional).
    /// - `mintings_info`: A list of minting information objects (optional).
    ///    Each minting info contains the following fields:
    ///     - `policy`: The policy ID of the asset to be minted/burnt.
    ///     - `assets`: A list of tuples containing the asset name and the amount to be minted/burnt.
    ///     - `redeemer_cbor`: The cbor-encoded redeemer to the minting policy.
    /// - `witnesses`: A list of public keys of input owners.
    /// - `required_signers`: A list of payment hashes of the senders.
    /// - `validity_interval_start`: Start of the validity interval (optional).
    /// - `ttl`: Time to live (optional).
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "TX_INFO_JSON"
    )]
    pub tx_info: String,
}

/// Arguments for spending wallet inputs only.
#[derive(Clone, Debug, Args)]
pub struct SpendValueArgs {
    /// An input to be consumed by this transaction. This argument may be specified multiple times.
    #[arg(long, short, verbatim_doc_comment, value_parser = input_from_string, required = true, value_name = "OUTPUT_REF")]
    pub input: Vec<Input>,

    /// 32-byte H256 public key of an input owner.
    /// Their pk/sk pair must be registered in the wallet's keystore.
    #[arg(long, short, verbatim_doc_comment, value_parser = h256_from_string, default_value = SHAWN_PUB_KEY, value_name = "PUBLIC_KEY")]
    pub witness: Vec<H256>,

    /// 29-byte hash-address of the recipient.
    #[arg(long, short, verbatim_doc_comment, value_parser = address_from_string, default_value = SHAWN_ADDRESS, value_name = "ADDRESS")]
    pub recipient: Address,

    /// An amount of `Coin`s to be included in the output value.
    #[arg(long, short, verbatim_doc_comment, action = Append)]
    pub amount: Option<Coin>,

    /// Policy ID of the asset to be spent.
    #[arg(long, short, verbatim_doc_comment, value_parser = h224_from_string, action = Append, value_name = "POLICY_ID")]
    pub policy: Vec<PolicyId>,

    /// Name of the asset to be spent.
    #[arg(long, short, verbatim_doc_comment, action = Append, value_name = "ASSET_NAME")]
    pub name: Vec<String>,

    /// How many tokens of the given asset should be included.
    #[arg(long, short, verbatim_doc_comment, action = Append, value_name = "AMOUNT")]
    pub token_amount: Vec<Coin>,
}

#[derive(Clone, Debug, Args)]
pub struct ShowOutputsAtArgs {
    /// 29-byte hash-address.
    #[arg(long, short, verbatim_doc_comment, value_parser = address_from_string, required = true, value_name = "ADDRESS")]
    pub address: Address,
}

#[derive(Clone, Debug, Args)]
pub struct ShowOutputsWithAssetArgs {
    /// Policy ID of the asset.
    #[arg(long, short, verbatim_doc_comment, value_parser = h224_from_string, required = true, value_name = "POLICY_ID")]
    pub policy: PolicyId,

    /// Name of the asset.
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        required = true,
        value_name = "ASSET_NAME"
    )]
    pub name: String,
}
