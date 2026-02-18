#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod types;

use alloc::{fmt::Debug, vec, vec::Vec};
use authority_selection_inherents::CommitteeMember as CommitteeMemberOf;
use griffin_core::genesis::config_builder::CommitteeData;
use griffin_core::types::{AssetName, Datum, Output};
use griffin_core::utxo_set::TransparentUtxoSet;
use griffin_core::COMMITTEE_KEY;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sidechain_domain::cross_chain_app::Public as CrossChainPublic;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ed25519, sr25519};
use sp_session_validator_management::CommitteeMember as CommitteeMemberT;
use thiserror::Error;
use types::*;

pub type CommitteeMember = CommitteeMemberOf<CrossChainPublic, AuthKeys>;

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
    #[error("No datum found")]
    DatumMissing,
    #[error("Datum has wrong shape")]
    BadDatum,
}

#[derive(
    Clone, Encode, Decode, DecodeWithMemTracking, TypeInfo, MaxEncodedLen, Debug, PartialEq, Eq,
)]
pub struct AuthKeys {
    aura: AuraId,
    grandpa: GrandpaId,
    weight: u64,
}

impl From<(Vec<u8>, Vec<u8>, u64)> for AuthKeys {
    fn from(thruple: (Vec<u8>, Vec<u8>, u64)) -> AuthKeys {
        let mut aura_raw = [0u8; 32];
        aura_raw.copy_from_slice(&thruple.0);
        let aura_public = sr25519::Public::from_raw(aura_raw);

        let mut gran_raw = [0u8; 32];
        gran_raw.copy_from_slice(&thruple.1);
        let gran_public = ed25519::Public::from_raw(gran_raw);
        AuthKeys {
            aura: AuraId::from(aura_public),
            grandpa: GrandpaId::from(gran_public),
            weight: thruple.2,
        }
    }
}

/// In charge of parsing a datum to lookup for aura and grandpa keys.
fn parse_authorities(datum: Datum) -> Result<Vec<AuthKeys>, ConfigParsingErrors> {
    let decoded: CommitteeDatum = CommitteeDatum::from(datum);
    match decoded {
        CommitteeDatum::Ok { cmt } => Ok(cmt.into_iter().map(|mem| mem.authority_keys()).collect()),
        CommitteeDatum::MalformedCommitteeDatum => Err(ConfigParsingErrors::BadDatum),
    }
}

fn expect_unique(outputs: &Vec<Output>) -> Result<Output, ConfigParsingErrors> {
    match outputs.as_slice() {
        [o] => Ok(o.clone()),
        [] => Err(ConfigParsingErrors::EmptyOutputs),
        _ => Err(ConfigParsingErrors::MoreThanOneOutput),
    }
}

pub fn get_cmt_data() -> CommitteeData {
    sp_io::storage::get(COMMITTEE_KEY)
        .and_then(|d| CommitteeData::decode(&mut &*d).ok())
        .unwrap()
}

fn fetch_utxo(asset_name: AssetName, cmt: CommitteeData) -> Result<Output, ConfigParsingErrors> {
    let outputs = TransparentUtxoSet::peek_utxos_with_asset(&asset_name, &cmt.policy_id);
    let output = expect_unique(&outputs).unwrap();
    if output.address == cmt.address {
        Ok(output)
    } else {
        Err(ConfigParsingErrors::BadAddress)
    }
}

pub fn aura_authorities() -> Vec<AuraId> {
    let cmt = get_cmt_data();
    if let Some(datum) = fetch_utxo(cmt.clone().current_asset_name, cmt)
        .unwrap()
        .datum_option
    {
        let authority_keys = parse_authorities(datum).unwrap();
        authority_keys.into_iter().map(|keys| keys.aura).collect()
    } else {
        vec![]
    }
}

pub fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
    let cmt = get_cmt_data();
    if let Some(datum) = fetch_utxo(cmt.clone().current_asset_name, cmt)
        .unwrap()
        .datum_option
    {
        let authority_keys = parse_authorities(datum).unwrap();
        authority_keys
            .into_iter()
            .map(|keys| (keys.grandpa, keys.weight))
            .collect()
    } else {
        vec![]
    }
}
