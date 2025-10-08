use alloc::{vec, vec::Vec};
use core::str::FromStr;
use derive_new::new;
use griffin_core::h224::H224;
use griffin_core::types::{Address, AssetName, Input, Output, PolicyId};
use griffin_core::uplc::Hash;
use griffin_core::utxo_set::TransparentUtxoSetApi;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use parity_scale_codec::Decode;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

use crate::error::error_object_from;

extern crate alloc;

#[rpc(client, server, namespace = "utxorpc")]
pub trait TransparentUtxoSetRpcApi {
    #[method(name = "get_utxo")]
    fn peek_utxo(&self, input: Bytes) -> RpcResult<Output>;
    #[method(name = "get_utxo_by_address")]
    fn peek_utxos_by_address(&self, input: Bytes) -> RpcResult<Vec<Output>>;
    #[method(name = "get_utxo_with_asset")]
    fn peek_utxos_with_asset(
        &self,
        asset_name: String,
        raw_policy_id: String,
    ) -> RpcResult<Vec<Output>>;
}

#[derive(new)]
pub struct TransparentUtxoSetRpc<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TransparentUtxoSetRpcApiServer for TransparentUtxoSetRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + 'static,
    C::Api: griffin_core::utxo_set::TransparentUtxoSetApi<Block>,
{
    fn peek_utxo(&self, input_bytes: Bytes) -> RpcResult<Output> {
        let api = self.client.runtime_api();
        let best_block = self.client.info().best_hash;

        let input = match Input::decode(&mut &input_bytes[..]) {
            Ok(input) => input,
            Err(e) => return Err(error_object_from(e)),
        };
        match api.peek_utxo(best_block, &input) {
            Ok(Some(outxo)) => Ok(outxo),
            Ok(None) => Err(error_object_from("No UTxO Found")),
            Err(e) => Err(error_object_from(e)),
        }
    }

    fn peek_utxos_by_address(&self, addr_bytes: Bytes) -> RpcResult<Vec<Output>> {
        let api = self.client.runtime_api();
        let best_block = self.client.info().best_hash;

        let addr = Address::from(addr_bytes.to_vec());
        let utxos: Vec<Output> =
            api.peek_utxo_by_address(best_block, &addr).map_err(error_object_from)?;

        Ok(utxos)
    }

    fn peek_utxos_with_asset(
        &self,
        asset_name: String,
        raw_policy_id: String,
    ) -> RpcResult<Vec<Output>> {
        let api = self.client.runtime_api();
        let best_block = self.client.info().best_hash;

        let policy_id = match Hash::from_str(&raw_policy_id) {
            Ok(hash) => hash,
            Err(e) => return Err(error_object_from(e)),
        };

        let name = AssetName::from(asset_name);
        let policy = PolicyId::from(policy_id);
        let utxos: Vec<Output> = api
            .peek_utxo_with_asset(best_block, &name, &policy)
            .map_err(error_object_from)?;

        Ok(utxos)
    }
}
