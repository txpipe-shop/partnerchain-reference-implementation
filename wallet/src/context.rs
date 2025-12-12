use crate::{cli::Cli, rpc, sync};
use clap::Parser;
use griffin_core::uplc::tx::SlotConfig;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use std::path::PathBuf;

/// The default RPC endpoint for the wallet to connect to
pub const DEFAULT_ENDPOINT: &str = "http://localhost:9944";

pub struct Context<T: clap::Subcommand + clap::FromArgMatches> {
    pub cli: Cli<T>,
    pub client: HttpClient,
    pub db: sled::Db,
    pub keystore: sc_keystore::LocalKeystore,
    pub data_path: PathBuf,
    pub keystore_path: PathBuf,
    pub slot_config: SlotConfig,
}

impl<T: clap::Subcommand + clap::FromArgMatches> Context<T> {
    pub async fn load_context() -> anyhow::Result<Context<T>> {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

        // Parse command line args
        let cli = Cli::<T>::parse();

        // If the user specified --tmp or --dev, then use a temporary directory.
        let tmp = cli.tmp || cli.dev;

        // Setup the data paths.
        let data_path = match tmp {
            true => temp_dir(),
            _ => cli.base_path.clone().unwrap_or_else(default_data_path),
        };
        let keystore_path = data_path.join("keystore");
        let db_path = data_path.join("wallet_database");

        // Setup the keystore
        let keystore = sc_keystore::LocalKeystore::open(keystore_path.clone(), None)?;

        if cli.dev {
            // Insert the example Shawn key so example transactions can be signed.
            crate::keystore::insert_development_key_for_this_session(&keystore)?;
        }

        // Setup jsonrpsee and endpoint-related information.
        // https://github.com/paritytech/jsonrpsee/blob/master/examples/examples/http.rs
        let client = HttpClientBuilder::default().build(&cli.endpoint)?;

        // Read node's genesis block.
        let node_genesis_hash = rpc::node_get_block_hash(0, &client)
            .await?
            .expect("node should be able to return some genesis hash");
        let node_genesis_block = rpc::node_get_block(node_genesis_hash, &client)
            .await?
            .expect("node should be able to return some genesis block");
        log::debug!("Node's Genesis block::{:?}", node_genesis_hash);

        let zero_time = rpc::node_get_zero_time(&client)
            .await?
            .expect("node should be able to return zero time");
        let zero_slot = rpc::node_get_zero_slot(&client)
            .await?
            .expect("node should be able to return zero slot");
        let slot_length = rpc::node_get_slot_length(&client)
            .await?
            .expect("node should be able to return slot length");
        let slot_config = SlotConfig {
            zero_time,
            zero_slot,
            slot_length,
        };

        if cli.purge_db {
            std::fs::remove_dir_all(db_path.clone()).map_err(|e| {
                log::warn!(
                    "Unable to remove database directory at {}\nPlease remove it manually.",
                    db_path.to_string_lossy()
                );
                e
            })?;
        }

        // Open the local database
        let db = sync::open_db(db_path, node_genesis_hash, node_genesis_block.clone())?;

        let num_blocks =
            sync::height(&db)?.expect("db should be initialized automatically when opening.");
        log::info!("Number of blocks in the db: {num_blocks}");

        if !sled::Db::was_recovered(&db) {
            sync::apply_block(&db, node_genesis_block, node_genesis_hash).await?;
        }

        // Synchronize the wallet with attached node unless instructed otherwise.
        if cli.no_sync {
            log::warn!("Skipping sync with node. Using previously synced information.")
        } else {
            sync::synchronize(&db, &client).await?;

            log::info!(
                "Wallet database synchronized with node to height {:?}",
                sync::height(&db)?.expect("We just synced, so there is a height available")
            );
        };

        Ok(Context {
            cli,
            client,
            db,
            keystore,
            data_path,
            keystore_path,
            slot_config,
        })
    }
}

/// Generate a plaform-specific temporary directory for the wallet
fn temp_dir() -> PathBuf {
    // Since it is only used for testing purpose, we don't need a secure temp dir, just a unique one.
    std::env::temp_dir().join(format!(
        "gpc-wallet-{}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
    ))
}

/// Generate the platform-specific default data path for the wallet
fn default_data_path() -> PathBuf {
    // This uses the directories crate.
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html

    // Application developers may want to put actual qualifiers or organization here
    let qualifier = "";
    let organization = "";
    let application = env!("CARGO_PKG_NAME");

    directories::ProjectDirs::from(qualifier, organization, application)
        .expect("app directories exist on all supported platforms; qed")
        .data_dir()
        .into()
}
