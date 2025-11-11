//! Helper functions for communicating with the Node's RPC endpoint.

use griffin_core::types::{Input, OpaqueBlock, Output};
use griffin_core::{SLOT_LENGTH, ZERO_SLOT, ZERO_TIME};
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use parity_scale_codec::Encode;
use sp_core::H256;

/// Get the Node's block hash at a particular height
pub async fn node_get_block_hash(height: u32, client: &HttpClient) -> anyhow::Result<Option<H256>> {
    let params = rpc_params![Some(height)];
    let rpc_response: Option<String> = client.request("chain_getBlockHash", params).await?;
    let maybe_hash = rpc_response.map(|s| crate::utils::h256_from_string(&s).unwrap());
    Ok(maybe_hash)
}

/// Get the node's full opaque block at a particular hash
pub async fn node_get_block(
    hash: H256,
    client: &HttpClient,
) -> anyhow::Result<Option<OpaqueBlock>> {
    let s = hex::encode(hash.0);
    let params = rpc_params![s];

    let maybe_rpc_response: Option<serde_json::Value> =
        client.request("chain_getBlock", params).await?;
    let rpc_response = maybe_rpc_response.unwrap();

    let json_opaque_block = rpc_response.get("block").cloned().unwrap();
    let opaque_block: OpaqueBlock = serde_json::from_value(json_opaque_block).unwrap();

    Ok(Some(opaque_block))
}

/// Fetch an output from chain storage given an Input
pub async fn fetch_storage(input: &Input, client: &HttpClient) -> anyhow::Result<Output> {
    let ref_hex = hex::encode(input.encode());
    let params = rpc_params![ref_hex];
    let routput: Result<Output, _> = client.request("utxorpc_get_utxo", params).await;
    let utxo = routput?;
    Ok(utxo)
}

/// Get the Node's initial POSIX time
pub async fn node_get_zero_time(client: &HttpClient) -> anyhow::Result<Option<u64>> {
    let params = rpc_params![hex::encode(str::from_utf8(ZERO_TIME).unwrap())];
    let rpc_response: Option<String> = client.request("state_getStorage", params).await?;
    let time_bytes: [u8; 8] = hex::decode(rpc_response.unwrap().strip_prefix("0x").unwrap()).unwrap().try_into().unwrap();
    let time = u64::from_le_bytes(time_bytes);
    Ok(Some(time))
}

/// Get the Node's zero slot
pub async fn node_get_zero_slot(client: &HttpClient) -> anyhow::Result<Option<u64>> {
    let params = rpc_params![hex::encode(str::from_utf8(ZERO_SLOT).unwrap())];
    let rpc_response: Option<String> = client.request("state_getStorage", params).await?;
    let slot_bytes: [u8; 8] = hex::decode(rpc_response.unwrap().strip_prefix("0x").unwrap()).unwrap().try_into().unwrap();
    let slot = u64::from_le_bytes(slot_bytes);
    Ok(Some(slot))
}

/// Get the Node's slot length
pub async fn node_get_slot_length(client: &HttpClient) -> anyhow::Result<Option<u32>> {
    let params = rpc_params![hex::encode(str::from_utf8(SLOT_LENGTH).unwrap())];
    let rpc_response: Option<String> = client.request("state_getStorage", params).await?;
    let slot_length_bytes: [u8; 4] = hex::decode(rpc_response.unwrap().strip_prefix("0x").unwrap()).unwrap().try_into().unwrap();
    let slot_length = u32::from_le_bytes(slot_length_bytes);
    Ok(Some(slot_length))
}
