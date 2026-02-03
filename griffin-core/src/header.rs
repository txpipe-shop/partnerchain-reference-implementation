use crate::{DATA_KEY, genesis::config_builder::PartnerChainData};
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{
    generic::Digest,
    traits::{BlakeTwo256, Hash as HashT, Header as HeaderT},
};

pub type Hash = BlakeTwo256;
pub type OpaqueHash = <Hash as HashT>::Output;
pub type BlockNumber = u32;

#[derive(
    Encode,
    Decode,
    DecodeWithMemTracking,
    Debug,
    PartialEq,
    Eq,
    Clone,
    TypeInfo,
    Serialize,
    Deserialize,
)]
pub struct ExtendedHeader{
    pub parent_hash: OpaqueHash,
    pub number: BlockNumber,
    pub state_root: OpaqueHash,
    pub extrinsics_root: OpaqueHash,
    pub digest: Digest,
    pub data: Option<PartnerChainData>,
}

impl ExtendedHeader {
    pub fn new(
        number: BlockNumber,
        extrinsics_root: OpaqueHash,
        state_root: OpaqueHash,
        parent_hash: OpaqueHash,
        digest: Digest,
    ) -> Self {
        Self {
            number,
            extrinsics_root,
            state_root,
            parent_hash,
            digest,
            data: None,
        }
    }

    pub fn get_pcdata(&self) -> &Option<PartnerChainData> {
        &self.data
    }

    pub fn get_pcdata_storage() -> Option<PartnerChainData> {
        sp_io::storage::get(DATA_KEY).and_then(|d| PartnerChainData::decode(&mut &*d).ok())
    }

    pub fn set_pcdata(&mut self, data: PartnerChainData) {
        self.data = Some(data)
    }
}

impl HeaderT for ExtendedHeader {
    type Number = BlockNumber;
    type Hash = OpaqueHash;
    type Hashing = Hash;

    fn new(
        number: Self::Number,
        extrinsics_root: Self::Hash,
        state_root: Self::Hash,
        parent_hash: Self::Hash,
        digest: Digest,
    ) -> Self {
        Self::new(number, extrinsics_root, state_root, parent_hash, digest)
    }
    fn number(&self) -> &Self::Number {
        &self.number
    }

    fn set_number(&mut self, num: Self::Number) {
        self.number = num
    }
    fn extrinsics_root(&self) -> &Self::Hash {
        &self.extrinsics_root
    }

    fn set_extrinsics_root(&mut self, root: Self::Hash) {
        self.extrinsics_root = root
    }
    fn state_root(&self) -> &Self::Hash {
        &self.state_root
    }

    fn set_state_root(&mut self, root: Self::Hash) {
        self.state_root = root
    }
    fn parent_hash(&self) -> &Self::Hash {
        &self.parent_hash
    }

    fn set_parent_hash(&mut self, hash: Self::Hash) {
        self.parent_hash = hash
    }

    fn digest(&self) -> &Digest {
        &self.digest
    }

    fn digest_mut(&mut self) -> &mut Digest {
        #[cfg(feature = "std")]
        log::debug!(target: "header", "Retrieving mutable reference to digest");
        &mut self.digest
    }
}
