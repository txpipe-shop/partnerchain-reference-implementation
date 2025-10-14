//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use griffin_rpc::cardano_rpc::{CardanoRpc, CardanoRpcApiServer};
use griffin_rpc::rpc::{TransparentUtxoSetRpc, TransparentUtxoSetRpcApiServer};
use jsonrpsee::RpcModule;
use sc_transaction_pool_api::TransactionPool;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use std::sync::Arc;

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
}

#[docify::export]
/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: Send
        + Sync
        + 'static
        + sp_api::ProvideRuntimeApi<<P as sc_transaction_pool_api::TransactionPool>::Block>
        + HeaderBackend<<P as sc_transaction_pool_api::TransactionPool>::Block>
        + HeaderMetadata<
            <P as sc_transaction_pool_api::TransactionPool>::Block,
            Error = BlockChainError,
        >
        + 'static,
    C::Api: BlockBuilder<<P as sc_transaction_pool_api::TransactionPool>::Block>,
    C::Api: griffin_core::utxo_set::TransparentUtxoSetApi<
        <P as sc_transaction_pool_api::TransactionPool>::Block,
    >,
    P: TransactionPool + 'static,
{
    let mut module = RpcModule::new(());
    let FullDeps { client, pool } = deps;

    module.merge(CardanoRpc::new(client.clone(), pool.clone()).into_rpc())?;
    module.merge(TransparentUtxoSetRpc::new(client.clone()).into_rpc())?;

    Ok(module)
}
