use alloc::vec::Vec;
use griffin_core::{
    pallas_codec::{
        minicbor,
        utils::{Int, MaybeIndefArray::Indef},
    },
    pallas_primitives::babbage::{BigInt, BoundedBytes, Constr, PlutusData as PallasPlutusData},
    types::{Datum, PlutusData},
};
use sp_core::ByteArray as _;

use crate::CommitteeMember;

fn from_raw(id: Vec<u8>, keys: (Vec<u8>, Vec<u8>, u64)) -> CommitteeMember {
    CommitteeMember::Permissioned {
        id: id.into(),
        keys: keys.into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommitteeDatum {
    Ok { cmt: Vec<CommitteeMember> },
    MalformedCommitteeDatum,
}

impl From<CommitteeDatum> for Datum {
    fn from(ship_datum: CommitteeDatum) -> Self {
        Datum(PlutusData::from(PallasPlutusData::from(ship_datum)).0)
    }
}

impl From<Datum> for CommitteeDatum {
    fn from(datum: Datum) -> Self {
        <_>::from(PallasPlutusData::from(PlutusData(datum.0)))
    }
}

impl From<CommitteeDatum> for PallasPlutusData {
    fn from(ship_datum: CommitteeDatum) -> Self {
        match ship_datum {
            CommitteeDatum::Ok { cmt } => PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [PallasPlutusData::Array(Indef(
                        cmt.into_iter()
                            .map(|mem| PallasPlutusData::from(CommitteeMemberWrapper(mem)))
                            .collect(),
                    ))]
                    .to_vec(),
                ),
            }),
            CommitteeDatum::MalformedCommitteeDatum => {
                PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(-1))))
            }
        }
    }
}

impl From<PallasPlutusData> for CommitteeDatum {
    fn from(data: PallasPlutusData) -> Self {
        if let PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef(datum),
        }) = data
        {
            if let [PallasPlutusData::Array(Indef(vec))] = &datum.clone()[..] {
                CommitteeDatum::Ok {
                    cmt: vec
                        .clone()
                        .into_iter()
                        .map(|mem| CommitteeMemberWrapper::try_from(mem).unwrap().0)
                        .collect(),
                }
            } else {
                CommitteeDatum::MalformedCommitteeDatum
            }
        } else {
            CommitteeDatum::MalformedCommitteeDatum
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommitteeMemberWrapper(CommitteeMember);

impl From<CommitteeMemberWrapper> for PallasPlutusData {
    fn from(member: CommitteeMemberWrapper) -> Self {
        match member.0 {
            CommitteeMember::Permissioned { id, keys } => PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [
                        PallasPlutusData::BoundedBytes(BoundedBytes(id.to_raw_vec())),
                        PallasPlutusData::Constr(Constr {
                            tag: 121,
                            any_constructor: None,
                            fields: Indef(
                                [
                                    PallasPlutusData::BoundedBytes(BoundedBytes(
                                        keys.aura.to_raw_vec(),
                                    )),
                                    PallasPlutusData::BoundedBytes(BoundedBytes(
                                        keys.grandpa.to_raw_vec(),
                                    )),
                                    PallasPlutusData::BigInt(BigInt::Int(Int(
                                        minicbor::data::Int::from(keys.weight),
                                    ))),
                                ]
                                .to_vec(),
                            ),
                        }),
                    ]
                    .to_vec(),
                ),
            }),
            CommitteeMember::Registered { .. } => PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef([].to_vec()),
            }),
        }
    }
}

impl TryFrom<PallasPlutusData> for CommitteeMemberWrapper {
    type Error = &'static str;

    fn try_from(data: PallasPlutusData) -> Result<Self, Self::Error> {
        if let PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef(mem),
        }) = data
        {
            if let [PallasPlutusData::BoundedBytes(BoundedBytes(id)), PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(ref fields),
            })] = &mem[..]
            {
                if let [PallasPlutusData::BoundedBytes(BoundedBytes(aura)), PallasPlutusData::BoundedBytes(BoundedBytes(gran)), PallasPlutusData::BigInt(BigInt::Int(Int(weight)))] =
                    &fields[..]
                {
                    Ok(CommitteeMemberWrapper(from_raw(
                        id.clone(),
                        (
                            aura.clone(),
                            gran.clone(),
                            TryFrom::<minicbor::data::Int>::try_from(*weight).unwrap(),
                        ),
                    )))
                } else {
                    Err("Failed to parse AuthKeys from Plutus Data")
                }
            } else {
                Err("Failed to parse Id or AuthKeys from Plutus Data")
            }
        } else {
            Err("Failed to parse Committee Member from Plutus Data")
        }
    }
}
