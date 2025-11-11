use griffin_core::{
    h224::H224,
    types::{Address, Input, Value},
};
use parity_scale_codec::Decode;
use sp_core::H256;

use crate::rpc;

/// Parse a string into an H256 that represents a public key
pub fn h256_from_string(s: &str) -> anyhow::Result<H256> {
    let s = strip_0x_prefix(s);

    let mut bytes: [u8; 32] = [0; 32];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(H256::from(bytes))
}

/// Parse a string into an H224 that represents a policy ID.
pub(crate) fn h224_from_string(s: &str) -> anyhow::Result<H224> {
    let s = strip_0x_prefix(s);

    let mut bytes: [u8; 28] = [0; 28];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(H224::from(bytes))
}

/// Parse a string into an Address that represents a public key
pub(crate) fn address_from_string(s: &str) -> anyhow::Result<Address> {
    let s = strip_0x_prefix(s);

    let mut bytes: [u8; 29] = [0; 29];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(Address(Vec::from(bytes)))
}

/// Parse an output ref from a string
pub fn input_from_string(s: &str) -> Result<Input, clap::Error> {
    let s = strip_0x_prefix(s);
    let bytes =
        hex::decode(s).map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;

    Input::decode(&mut &bytes[..])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))
}

/// Takes a string and checks for a 0x prefix. Returns a string without a 0x prefix.
fn strip_0x_prefix(s: &str) -> &str {
    if &s[..2] == "0x" {
        &s[2..]
    } else {
        s
    }
}

/// Given an output ref, fetch the details about its value from the node's
/// storage.
pub async fn get_coin_from_storage(
    input: &Input,
    client: &jsonrpsee::http_client::HttpClient,
) -> anyhow::Result<Value> {
    let utxo = rpc::fetch_storage(input, client).await?;
    let coin_in_storage: Value = utxo.value;

    Ok(coin_in_storage)
}
