use alloc::{format, string::String, vec, vec::Vec};
use griffin_core::types::Block;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::Deserialize;
use sp_io::offchain;
use sp_runtime::traits::Block as BlockT;

const LOG_TARGET: &str = "offchain-worker";
const DOLOS_ENDPOINT: &str = "http://localhost:3000";

#[derive(Encode, Decode, PartialEq, Eq, Clone, TypeInfo, Debug, Deserialize)]
struct DolosAmount {
    unit: String,
    quantity: String,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, TypeInfo, Debug, Deserialize)]
struct DolosUTxO {
    address: String,
    tx_hash: String,
    tx_index: u32,
    output_index: u32,
    amount: Vec<DolosAmount>,
    block: String,
    data_hash: String,
    inline_datum: String,
    reference_script_hash: Option<String>,
}

pub fn offchain_worker(_header: &<Block as BlockT>::Header) {
    let address = "addr_test1wz03nr0ds4h9ym274m63sdvn5wqegp0khv2yet9v6av3tpg25x8q3";
    let query_endpoint = format!("{}/addresses/{}/utxos", DOLOS_ENDPOINT, address);

    let id = offchain::http_request_start("GET", &query_endpoint, &[])
        .expect("Offchain worker failed to start HTTP request.");
    offchain::http_response_wait(&[id], None);

    let mut buffer = vec![0_u8; 16384]; // 16k read buffer;
    let read = offchain::http_response_read_body(id, &mut buffer, None)
        .expect("Offchain worker failed to read HTTP response body.");
    let utxos = serde_json::from_slice::<Vec<DolosUTxO>>(&buffer[0..read as usize])
        .expect("Offchain worker failed to parse UTxOs from HTTP response body.");

    if utxos.is_empty() {
        log::warn!(
            target: LOG_TARGET,
            "\n\n Offchain worker fetched no UTxOs \n",
        );
    } else {
        log::warn!(
            target: LOG_TARGET,
            "\n\n Offchain worker fetched UTxO: {:?} \n",
            utxos[0],
        );
    }
}
