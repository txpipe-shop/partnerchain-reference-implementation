use griffin_core::{
    h224::H224,
    pallas_codec::{
        minicbor,
        utils::{Int, MaybeIndefArray::Indef},
    },
    pallas_crypto::hash::Hash as PallasHash,
    pallas_primitives::babbage::{BigInt, BoundedBytes, Constr, PlutusData as PallasPlutusData},
    types::{AssetName, Datum, PlutusData, PolicyId},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ScriptsParams {
    pub admin_policy: String,
    pub admin_name: String,
    pub fuel_per_step: u64,
    pub initial_fuel: u64,
    pub max_speed: Speed,
    pub max_ship_fuel: u64,
    pub max_asteria_mining: u64,
    pub min_asteria_distance: u64,
    pub ship_mint_lovelace_fee: u64,
    pub scripts_directory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Speed {
    pub distance: u64,
    pub time: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AsteriaDatum {
    Ok {
        ship_counter: u16,
        shipyard_policy: PolicyId,
    },
    MalformedAsteriaDatum,
}

impl From<AsteriaDatum> for Datum {
    fn from(asteria_datum: AsteriaDatum) -> Self {
        Datum(PlutusData::from(PallasPlutusData::from(asteria_datum)).0)
    }
}

impl From<Datum> for AsteriaDatum {
    fn from(datum: Datum) -> Self {
        <_>::from(PallasPlutusData::from(PlutusData(datum.0)))
    }
}

impl From<AsteriaDatum> for PallasPlutusData {
    fn from(asteria_datum: AsteriaDatum) -> Self {
        match asteria_datum {
            AsteriaDatum::Ok {
                ship_counter,
                shipyard_policy,
            } => PallasPlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [
                        PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(
                            ship_counter,
                        )))),
                        PallasPlutusData::BoundedBytes(BoundedBytes(shipyard_policy.0.to_vec())),
                    ]
                    .to_vec(),
                ),
            })),
            AsteriaDatum::MalformedAsteriaDatum => {
                PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(-1))))
            }
        }
    }
}

impl From<PallasPlutusData> for AsteriaDatum {
    fn from(data: PallasPlutusData) -> Self {
        if let PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef(asteria_datum),
        }) = data
        {
            if let [PallasPlutusData::BigInt(BigInt::Int(Int(ship_counter))), PallasPlutusData::BoundedBytes(BoundedBytes(shipyard_policy_vec))] =
                &asteria_datum[..]
            {
                AsteriaDatum::Ok {
                    ship_counter: TryFrom::<minicbor::data::Int>::try_from(*ship_counter).unwrap(),
                    shipyard_policy: H224::from(PallasHash::from(shipyard_policy_vec.as_slice())),
                }
            } else {
                AsteriaDatum::MalformedAsteriaDatum
            }
        } else {
            AsteriaDatum::MalformedAsteriaDatum
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PelletDatum {
    Ok {
        pos_x: i16,
        pos_y: i16,
        shipyard_policy: PolicyId,
    },
    MalformedPelletDatum,
}

impl From<PelletDatum> for Datum {
    fn from(pellet_datum: PelletDatum) -> Self {
        Datum(PlutusData::from(PallasPlutusData::from(pellet_datum)).0)
    }
}

impl From<Datum> for PelletDatum {
    fn from(datum: Datum) -> Self {
        <_>::from(PallasPlutusData::from(PlutusData(datum.0)))
    }
}

impl From<PelletDatum> for PallasPlutusData {
    fn from(pellet_datum: PelletDatum) -> Self {
        match pellet_datum {
            PelletDatum::Ok {
                pos_x,
                pos_y,
                shipyard_policy,
            } => PallasPlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [
                        PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(
                            pos_x,
                        )))),
                        PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(
                            pos_y,
                        )))),
                        PallasPlutusData::BoundedBytes(BoundedBytes(shipyard_policy.0.to_vec())),
                    ]
                    .to_vec(),
                ),
            })),
            PelletDatum::MalformedPelletDatum => {
                PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(-1))))
            }
        }
    }
}

impl From<PallasPlutusData> for PelletDatum {
    fn from(data: PallasPlutusData) -> Self {
        if let PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef(pellet_datum),
        }) = data
        {
            if let [PallasPlutusData::BigInt(BigInt::Int(Int(pos_x))), PallasPlutusData::BigInt(BigInt::Int(Int(pos_y))), PallasPlutusData::BoundedBytes(BoundedBytes(shipyard_policy_vec))] =
                &pellet_datum[..]
            {
                PelletDatum::Ok {
                    pos_x: TryFrom::<minicbor::data::Int>::try_from(*pos_x).unwrap(),
                    pos_y: TryFrom::<minicbor::data::Int>::try_from(*pos_y).unwrap(),
                    shipyard_policy: H224::from(PallasHash::from(shipyard_policy_vec.as_slice())),
                }
            } else {
                PelletDatum::MalformedPelletDatum
            }
        } else {
            PelletDatum::MalformedPelletDatum
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ShipDatum {
    Ok {
        pos_x: i16,
        pos_y: i16,
        ship_token_name: AssetName,
        pilot_token_name: AssetName,
        last_move_latest_time: u64,
    },
    MalformedShipDatum,
}

impl From<ShipDatum> for Datum {
    fn from(ship_datum: ShipDatum) -> Self {
        Datum(PlutusData::from(PallasPlutusData::from(ship_datum)).0)
    }
}

impl From<Datum> for ShipDatum {
    fn from(datum: Datum) -> Self {
        <_>::from(PallasPlutusData::from(PlutusData(datum.0)))
    }
}

impl From<ShipDatum> for PallasPlutusData {
    fn from(ship_datum: ShipDatum) -> Self {
        match ship_datum {
            ShipDatum::Ok {
                pos_x,
                pos_y,
                ship_token_name,
                pilot_token_name,
                last_move_latest_time,
            } => PallasPlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [
                        PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(
                            pos_x,
                        )))),
                        PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(
                            pos_y,
                        )))),
                        PallasPlutusData::BoundedBytes(BoundedBytes(
                            ship_token_name.0.clone().into(),
                        )),
                        PallasPlutusData::BoundedBytes(BoundedBytes(
                            pilot_token_name.0.clone().into(),
                        )),
                        PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(
                            last_move_latest_time,
                        )))),
                    ]
                    .to_vec(),
                ),
            })),
            ShipDatum::MalformedShipDatum => {
                PallasPlutusData::BigInt(BigInt::Int(Int(minicbor::data::Int::from(-1))))
            }
        }
    }
}

impl From<PallasPlutusData> for ShipDatum {
    fn from(data: PallasPlutusData) -> Self {
        if let PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef(ship_datum),
        }) = data
        {
            if let [PallasPlutusData::BigInt(BigInt::Int(Int(pos_x))), PallasPlutusData::BigInt(BigInt::Int(Int(pos_y))), PallasPlutusData::BoundedBytes(BoundedBytes(ship_name_vec)), PallasPlutusData::BoundedBytes(BoundedBytes(pilot_name_vec)), PallasPlutusData::BigInt(BigInt::Int(Int(last_move_latest_time)))] =
                &ship_datum[..]
            {
                ShipDatum::Ok {
                    pos_x: TryFrom::<minicbor::data::Int>::try_from(*pos_x).unwrap(),
                    pos_y: TryFrom::<minicbor::data::Int>::try_from(*pos_y).unwrap(),
                    ship_token_name: AssetName(String::from_utf8(ship_name_vec.to_vec()).unwrap()),
                    pilot_token_name: AssetName(
                        String::from_utf8(pilot_name_vec.to_vec()).unwrap(),
                    ),
                    last_move_latest_time: TryFrom::<minicbor::data::Int>::try_from(
                        *last_move_latest_time,
                    )
                    .unwrap(),
                }
            } else {
                ShipDatum::MalformedShipDatum
            }
        } else {
            ShipDatum::MalformedShipDatum
        }
    }
}
