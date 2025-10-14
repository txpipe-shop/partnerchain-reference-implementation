#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{string::ToString, vec, vec::Vec};
use core::str::FromStr;
use griffin_core::h224::H224;
use griffin_core::pallas_codec::minicbor;
use griffin_core::pallas_codec::utils::AnyCbor;
use griffin_core::pallas_primitives::{Bytes, MaybeIndefArray};
use griffin_core::types::{Address, AssetName, Datum, Output};
use griffin_core::uplc::Hash;
use griffin_core::utxo_set::TransparentUtxoSet;
use hex_literal::hex;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::ByteArray;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigParsingErrors {
    #[error("Expecting indef array")]
    BadPlutusData,
    #[error("No outputs found")]
    EmptyOutputs,
    #[error("Expected only one output")]
    MoreThanOneOutput,
    #[error("NFT not owned by the expected address")]
    BadAddress,
}

const AUTHORITIES_ADDRESS: &[u8] =
    &hex!("0000000000000000000000000000000000000000000000000000000000");
pub const RAW_AUTHORITIES_TOKEN_NAME: &str = "Authorities";
pub const RAW_AUTHORITIES_POLICY_ID: &str =
    "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005";

/// In charge of parsing a datum to lookup for aura and grandpa keys.
fn parse_authorities(bytes: &[u8]) -> Result<(Vec<Bytes>, Vec<Bytes>), ConfigParsingErrors> {
    let mut aura_keys = vec![];
    let mut grandpa_keys = vec![];
    if let Ok(MaybeIndefArray::Indef(arr0)) = minicbor::decode::<MaybeIndefArray<AnyCbor>>(bytes) {
        if let Ok(MaybeIndefArray::Indef(arr1)) =
            minicbor::decode::<MaybeIndefArray<AnyCbor>>(arr0[1].raw_bytes())
        {
            for arr11 in arr1.iter() {
                if let Ok(MaybeIndefArray::Indef(arr2)) =
                    minicbor::decode::<MaybeIndefArray<Bytes>>(arr11.raw_bytes())
                {
                    aura_keys.push(arr2[1].clone());
                    grandpa_keys.push(arr2[2].clone());
                } else {
                    return Err(ConfigParsingErrors::BadPlutusData);
                }
            }
        } else {
            return Err(ConfigParsingErrors::BadPlutusData);
        }
    } else {
        return Err(ConfigParsingErrors::BadPlutusData);
    }
    Ok((aura_keys, grandpa_keys))
}

fn expect_unique(outputs: &Vec<Output>) -> Result<Output, ConfigParsingErrors> {
    match outputs.as_slice() {
        [o] => Ok(o.clone()),
        [] => Err(ConfigParsingErrors::EmptyOutputs),
        _ => Err(ConfigParsingErrors::MoreThanOneOutput),
    }
}

fn fetch_utxo_datum() -> Result<Datum, ConfigParsingErrors> {
    let asset_name: AssetName = AssetName::from(RAW_AUTHORITIES_TOKEN_NAME.to_string());
    let policy_id: H224 = H224::from(Hash::from_str(RAW_AUTHORITIES_POLICY_ID).unwrap());
    let authorities_addr: Address = Address::from(AUTHORITIES_ADDRESS.to_vec());

    let outputs = TransparentUtxoSet::peek_utxos_with_asset(&asset_name, &policy_id);
    let output = expect_unique(&outputs).unwrap();
    if output.address == authorities_addr {
        Ok(output.datum_option.clone().expect("Missing Inline Datum"))
    } else {
        Err(ConfigParsingErrors::BadAddress)
    }
}

pub fn aura_authorities() -> Vec<AuraId> {
    let Datum(ref datum) = fetch_utxo_datum().unwrap();
    let (key_bytes, _) = parse_authorities(datum).unwrap();
    key_bytes
        .iter()
        .map(|b| {
            AuraId::from_slice(b.as_ref()).expect("Invalid Aura authority hex/bytes was provided")
        })
        .collect()
}

pub fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
    let Datum(ref datum) = fetch_utxo_datum().unwrap();
    let (_, key_bytes) = parse_authorities(datum).unwrap();
    key_bytes
        .iter()
        .map(|b| {
            (
                GrandpaId::from_slice(b.as_ref())
                    .expect("Invalid Grandpa authority hex was provided"),
                1,
            )
        })
        .collect()
}
