//! Test Wallet's Command Line Interface.

use std::path::PathBuf;

use crate::{
    utils::{address_from_string, h224_from_string, h256_from_string, input_from_string},
    keystore::{SHAWN_ADDRESS, SHAWN_PUB_KEY},
    context::DEFAULT_ENDPOINT,
};
use clap::{ArgAction::Append, Args, Parser, Subcommand};
use griffin_core::types::{Address, Coin, Input, PolicyId};
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
#[derive(Debug, Subcommand)]
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
#[derive(Debug, Args)]
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
#[derive(Debug, Args)]
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

#[derive(Debug, Args)]
pub struct ShowOutputsAtArgs {
    /// 29-byte hash-address.
    #[arg(long, short, verbatim_doc_comment, value_parser = address_from_string, required = true, value_name = "ADDRESS")]
    pub address: Address,
}

#[derive(Debug, Args)]
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
