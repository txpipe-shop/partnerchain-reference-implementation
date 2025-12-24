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
//! ./target/release/gpc-wallet wallet show-all-outputs
//! ```

mod cli;
mod command;
mod context;
mod keystore;
mod order_book;
mod rpc;
mod sync;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cli::WalletCommand::Wallet(cli::Command::ShowAllOutputs);
    cmd.run().await
}
