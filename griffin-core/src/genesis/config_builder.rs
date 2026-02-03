//! Custom GenesisConfigBuilder, to allow extrinsics to be added to the genesis block.

use crate::{
    ensure,
    h224::H224,
    pallas_crypto::hash::Hash,
    types::{
        Address, address_from_hex, AssetName, Coin, EncapBTree, Input, Multiasset, Output, Transaction,
    },
    COMMITTEE_KEY, DATA_KEY, EXTRINSIC_KEY, SLOT_LENGTH, UTXO_SET, ZERO_SLOT, ZERO_TIME,
};
use alloc::{
    collections::BTreeMap,
    string::{String},
    vec,
    vec::Vec,
};
use core::str::FromStr;
use hex::FromHex;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sidechain_domain::{PolicyId, UtxoId};
use sp_io::hashing::twox_128;
use sp_runtime::traits::Hash as HashT;

pub struct GriffinGenesisConfigBuilder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransparentMultiasset<A> {
    pub policy: String,
    pub assets: Vec<(String, A)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransparentOutput {
    pub address: String,
    pub coin: Coin,
    pub value: Vec<TransparentMultiasset<Coin>>,
    pub datum: Option<String>,
}

/// Initial configuration for Partner Chain related information.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    Debug,
    PartialEq,
    Eq,
)]
pub struct PartnerChainData {
    pub genesis_utxo: UtxoId,
    pub d_parameter_policy: PolicyId,
    pub permissioned_policy: PolicyId,
    pub candidates_address: PolicyId,
}

/// Initial configuration for the block producing committee identifiers.
/// It includes the expected address and asset class of the committee utxo.
/// Also, we have a field dedicated to storing the UTxO for the next committee, which can be
/// updated at any point.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    Debug,
    PartialEq,
    Eq,
)]
pub struct CommitteeData {
    pub address: Address,
    pub current_asset_name: AssetName,
    pub next_asset_name: AssetName,
    pub policy_id: H224,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    Debug,
    PartialEq,
    Eq,
)]
pub struct CommitteeDataUnparsed {
    pub address: String,
    pub current_asset_name: String,
    pub next_asset_name: String,
    pub policy_id: String,
}

/// Genesis configuration for the Griffin chain.
/// It contains a list of outputs used to build the transactions
/// to be included in the genesis block, the initial slot and the initial time.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenesisConfig {
    pub zero_slot: u64,
    pub zero_time: u64,
    pub slot_length: u32,
    pub partner_chain_data: Option<PartnerChainData>,
    pub committee_data: CommitteeDataUnparsed,
    pub outputs: Vec<TransparentOutput>,
}

impl GriffinGenesisConfigBuilder
where
    Transaction: Encode,
{
    /// This function expects the chain's genesis configuration to be passed as a parameter.
    /// It will build the genesis block by creating transactions from the outputs provided
    /// in the genesis configuration. The transactions are created with empty inputs.
    /// It will also store the zero slot and zero time in the storage.
    pub fn build(genesis_config: GenesisConfig) -> sp_genesis_builder::Result {
        let transactions = vec![Transaction::from((
            vec![],
            genesis_config
                .outputs
                .into_iter()
                .map(transp_to_output)
                .collect(),
        ))];

        let cmt_data: CommitteeData = CommitteeData {
            address: Address::from(hex::decode(genesis_config.committee_data.address).unwrap()),
            current_asset_name: AssetName::from(genesis_config.committee_data.current_asset_name),
            next_asset_name: AssetName::from(genesis_config.committee_data.next_asset_name),
            policy_id: H224::from(Hash::from_str(&genesis_config.committee_data.policy_id).unwrap())
        };

        // The transactions, zero slot and zero time are stored under special keys.
        sp_io::storage::set(EXTRINSIC_KEY, &transactions.encode());
        sp_io::storage::set(ZERO_SLOT, &genesis_config.zero_slot.encode());
        sp_io::storage::set(ZERO_TIME, &genesis_config.zero_time.encode());
        sp_io::storage::set(SLOT_LENGTH, &genesis_config.slot_length.encode());
        sp_io::storage::set(DATA_KEY, &genesis_config.partner_chain_data.encode());
        sp_io::storage::set(COMMITTEE_KEY, &cmt_data.encode());

        for tx in transactions.into_iter() {
            // Enforce that transactions do not have any inputs.
            ensure!(
                tx.transaction_body.inputs.is_empty(),
                "Genesis transactions must not have any inputs."
            );
            // Insert the outputs into the storage.
            let tx_hash = sp_runtime::traits::BlakeTwo256::hash_of(&tx.encode());
            for (index, utxo) in tx.transaction_body.outputs.iter().enumerate() {
                // TODO: Thing of a way of use add_utxo_prefix function.
                let utxo_prefix = twox_128(UTXO_SET);
                let input = Input {
                    tx_hash,
                    index: index as u32,
                };
                let key = [&utxo_prefix[..], &input.encode()[..]].concat();
                sp_io::storage::set(&key, &utxo.encode());
            }
        }

        Ok(())
    }
}

pub fn transp_to_output(transp: TransparentOutput) -> Output {
    Output::from((
        address_from_hex(&transp.address),
        transp.coin,
        transp_to_multiasset(transp.value),
        transp
            .datum
            .map(|v| <_>::from(<Vec<u8>>::from_hex(v).unwrap())),
    ))
}

fn transp_to_assets<A>(transp: Vec<(String, A)>) -> EncapBTree<AssetName, A> {
    let mut asset_btree = BTreeMap::new();

    for (name, amount) in transp {
        asset_btree.insert(<_>::from(name), amount);
    }

    EncapBTree(asset_btree)
}

pub fn transp_to_multiasset<A>(transp: Vec<TransparentMultiasset<A>>) -> Multiasset<A> {
    let mut ma_btree = BTreeMap::new();

    for TransparentMultiasset { policy, assets } in transp {
        ma_btree.insert(
            H224::from(Hash::from_str(&policy).unwrap()),
            transp_to_assets(assets),
        );
    }

    EncapBTree(ma_btree)
}
