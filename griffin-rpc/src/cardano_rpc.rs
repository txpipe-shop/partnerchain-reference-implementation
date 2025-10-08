use derive_new::new;
use griffin_core::checks_interface::babbage_minted_tx_from_cbor_checked;
use griffin_core::pallas_primitives::babbage::Tx as BPallasTransaction;
use griffin_core::types::Transaction;
use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
};
use parity_scale_codec::{Decode, Encode};
use sc_transaction_pool_api::{error::IntoPoolError, TransactionPool, TransactionSource, TxHash};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use std::sync::Arc;

use crate::error::error_object_from;

const TX_SOURCE: TransactionSource = TransactionSource::External;

#[rpc(client, server, namespace = "cardano_utxorpc")]
pub trait CardanoRpcApi<Hash> {
    #[method(name = "submit_tx")]
    async fn submit_cardano_tx(&self, ext: Bytes) -> RpcResult<Hash>;
}

#[derive(new)]
pub struct CardanoRpc<C, P> {
    client: Arc<C>,
    pool: Arc<P>,
}

#[async_trait]
impl<C, P> CardanoRpcApiServer<TxHash<P>> for CardanoRpc<C, P>
where
    P: TransactionPool + Sync + Send + 'static,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<P::Block> + HeaderBackend<P::Block> + 'static,
{
    async fn submit_cardano_tx(&self, ctx_bytes: Bytes) -> RpcResult<TxHash<P>> {
        match babbage_minted_tx_from_cbor_checked(&ctx_bytes) {
            Ok(mtx) => {
                let ptx = BPallasTransaction::from(mtx);
                let tx = Transaction::from(ptx);
                let tx_bytes = Encode::encode(&tx);
                let xt = match Decode::decode(&mut &tx_bytes[..]) {
                    Ok(xt) => xt,
                    Err(err) => return Err(error_object_from(err)),
                };

                let best_block = self.client.info().best_hash;

                self.pool
                    .submit_one(best_block, TX_SOURCE, xt)
                    .await
                    .map_err(|e| {
                        e.into_pool_error()
                            .map(|e| error_object_from(e))
                            .unwrap_or_else(|e| error_object_from(e))
                            .into()
                    })
            }
            Err(e) => Err(error_object_from(e)),
        }
    }
}
