//! UTxO interface to storage.

use crate::{
    types::{Address, AssetName, Input, Output, PolicyId},
    LOG_TARGET, UTXO_SET,
};
use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use sp_io::hashing::twox_128;

pub struct TransparentUtxoSet;

mod api_declarations {
    use super::*;
    sp_api::decl_runtime_apis! {
        pub trait TransparentUtxoSetApi {
            fn peek_utxo(input: &Input) -> Option<Output>;
            fn peek_utxo_by_address(addr: &Address) -> Vec<Output>;
            fn peek_utxo_with_asset(asset_name: &AssetName, asset_policy: &PolicyId) -> Vec<Output>;
        }
    }
}
pub use api_declarations::*;

impl TransparentUtxoSet {
    /// Fetch a utxo from the set with matching addr.
    pub fn peek_utxos_from_address(addr: &Address) -> Vec<Output> {
        Self::peek_utxos_by(|o| o.address == *addr)
    }

    /// Fetch utxos from the set containing a given asset.
    pub fn peek_utxos_with_asset(asset_name: &AssetName, asset_policy: &PolicyId) -> Vec<Output> {
        Self::peek_utxos_by(|o| o.value.contains_asset(asset_name, asset_policy))
    }

    /// Fetch a utxo from the set.
    pub fn peek_utxo(input: &Input) -> Option<Output> {
        let key = Self::add_utxo_prefix(input);
        sp_io::storage::get(&key).and_then(|d| Output::decode(&mut &*d).ok())
    }

    /// Consume a Utxo from the set.
    pub fn consume_utxo(input: &Input) -> Option<Output> {
        let maybe_output = Self::peek_utxo(input);
        Self::remove_utxo(input);
        maybe_output
    }

    /// Add a utxo into the set.
    pub fn store_utxo(input: Input, output: &Output) {
        let key = Self::add_utxo_prefix(&input);
        log::debug!(
            target: LOG_TARGET,
            "Storing UTXO at key: {:?}",
            sp_core::hexdisplay::HexDisplay::from(&key)
        );
        sp_io::storage::set(&key, &output.encode());
    }

    /// Fetch a utxo from the set filtering by `f`.
    fn peek_utxos_by<F>(f: F) -> Vec<Output>
    where
        F: Fn(&Output) -> bool,
    {
        let mut outputs: Vec<Output> = vec![];

        let utxo_prefix = twox_128(UTXO_SET);
        let mut some_key = Some(utxo_prefix.to_vec());

        while let Some(key) = some_key.clone().filter(|k| k.starts_with(&utxo_prefix)) {
            if let Some(val) = sp_io::storage::get(&key) {
                if let Ok(o) = Output::decode(&mut &*val) {
                    if f(&o) {
                        outputs.push(o);
                    }
                }
            }
            some_key = sp_io::storage::next_key(&key);
        }

        return outputs;
    }

    fn remove_utxo(input: &Input) {
        let key = Self::add_utxo_prefix(&input);
        sp_io::storage::clear(&key);
    }

    fn add_utxo_prefix(input: &Input) -> Vec<u8> {
        let utxo_prefix = twox_128(UTXO_SET);
        [&utxo_prefix[..], &input.encode()[..]].concat()
    }
}
