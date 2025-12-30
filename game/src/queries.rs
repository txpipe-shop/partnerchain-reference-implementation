use crate::{types::*, ShowAsteriaArgs, ShowPelletsArgs, ShowShipsArgs};
use anyhow::anyhow;
use colored::Colorize;
use gpc_wallet::sync;
use griffin_core::types::{
    compute_plutus_v2_script_hash, Address, Datum, PlutusScript, PolicyId, Value,
};
use parity_scale_codec::Decode;
use sled::Db;

/// Show Asteria UTxO.
pub async fn show_asteria(db: &Db, args: ShowAsteriaArgs) -> anyhow::Result<()> {
    println!("\n###### ASTERIA ###########\n");

    let params_json: String = std::fs::read_to_string(args.params_path)
        .map_err(|e| anyhow!("Failed to read params file: {}", e))?;

    let params: ScriptsParams =
        serde_json::from_str(&params_json).map_err(|e| anyhow!("Invalid params JSON: {}", e))?;

    let asteria_script_hex: &str =
        &std::fs::read_to_string(params.scripts_directory.clone() + "asteria.txt")
            .map_err(|e| anyhow!("Failed to read asteria script: {}", e))?;

    let wallet_unspent_tree = db.open_tree(sync::UNSPENT)?;
    for x in wallet_unspent_tree.iter() {
        let (input_ivec, owner_amount_datum_ivec) = x?;
        let input = hex::encode(input_ivec);
        let (owner_pubkey, value, datum_option) =
            <(Address, Value, Option<Datum>)>::decode(&mut &owner_amount_datum_ivec[..])?;

        let asteria_script: PlutusScript =
            PlutusScript(hex::decode(asteria_script_hex).expect("Failed to decode asteria script"));
        let asteria_hash: PolicyId = compute_plutus_v2_script_hash(asteria_script.clone());
        let asteria_address: Address = Address(
            hex::decode("70".to_owned() + &hex::encode(asteria_hash))
                .map_err(|e| anyhow!("Failed to decode asteria address: {}", e))?,
        );

        if owner_pubkey == asteria_address {
            let asteria_datum = datum_option.map(|d| AsteriaDatum::from(d));
            match asteria_datum {
                None | Some(AsteriaDatum::MalformedAsteriaDatum) => {
                    println!(
                        "{}: datum {:?}, value: {}",
                        input,
                        asteria_datum,
                        value.normalize(),
                    );
                }
                Some(AsteriaDatum::Ok {
                    ship_counter,
                    shipyard_policy,
                }) => {
                    println!(
                        "{}:\n {} {:?}\n {} {:#?}\n {} {}\n",
                        input.bold(),
                        "SHIP COUNTER:".bold(),
                        ship_counter,
                        "SHIPYARD POLICY:".bold(),
                        shipyard_policy,
                        "VALUE:".bold(),
                        value.normalize(),
                    );
                }
            }
        }
    }

    Ok(())
}

/// Show pellet UTxOs.
pub async fn show_pellets(db: &Db, args: ShowPelletsArgs) -> anyhow::Result<()> {
    println!("\n###### PELLETS ###########\n");

    let params_json: String = std::fs::read_to_string(args.params_path)
        .map_err(|e| anyhow!("Failed to read params file: {}", e))?;

    let params: ScriptsParams =
        serde_json::from_str(&params_json).map_err(|e| anyhow!("Invalid params JSON: {}", e))?;

    let pellet_script_hex: &str =
        &std::fs::read_to_string(params.scripts_directory.clone() + "pellet.txt")
            .map_err(|e| anyhow!("Failed to read pellet script: {}", e))?;

    let wallet_unspent_tree = db.open_tree(sync::UNSPENT)?;
    for x in wallet_unspent_tree.iter() {
        let (input_ivec, owner_amount_datum_ivec) = x?;
        let input = hex::encode(input_ivec);
        let (owner_pubkey, value, datum_option) =
            <(Address, Value, Option<Datum>)>::decode(&mut &owner_amount_datum_ivec[..])?;

        let pellet_script: PlutusScript =
            PlutusScript(hex::decode(pellet_script_hex).expect("Failed to decode pellet script"));
        let pellet_hash: PolicyId = compute_plutus_v2_script_hash(pellet_script.clone());
        let pellet_address: Address = Address(
            hex::decode("70".to_owned() + &hex::encode(pellet_hash))
                .map_err(|e| anyhow!("Failed to decode pellet address: {}", e))?,
        );

        if owner_pubkey == pellet_address {
            let pellet_datum = datum_option.map(|d| PelletDatum::from(d));
            match pellet_datum {
                None | Some(PelletDatum::MalformedPelletDatum) => {
                    println!(
                        "{}: datum {:?}, value: {}",
                        input,
                        pellet_datum,
                        value.normalize(),
                    );
                }
                Some(PelletDatum::Ok {
                    pos_x,
                    pos_y,
                    shipyard_policy,
                }) => {
                    println!(
                        "{}:\n {} {:?}\n {} {:?}\n {} {:#?}\n {} {}\n",
                        input.bold(),
                        "POS X:".bold(),
                        pos_x,
                        "POS Y:".bold(),
                        pos_y,
                        "SHIPYARD POLICY:".bold(),
                        shipyard_policy,
                        "VALUE:".bold(),
                        value.normalize(),
                    );
                }
            }
        }
    }

    Ok(())
}

/// Show ship UTxOs.
pub async fn show_ships(db: &Db, args: ShowShipsArgs) -> anyhow::Result<()> {
    println!("\n###### SHIPS ###########\n");

    let params_json: String = std::fs::read_to_string(args.params_path)
        .map_err(|e| anyhow!("Failed to read params file: {}", e))?;

    let params: ScriptsParams =
        serde_json::from_str(&params_json).map_err(|e| anyhow!("Invalid params JSON: {}", e))?;

    let spacetime_script_hex: &str =
        &std::fs::read_to_string(params.scripts_directory.clone() + "spacetime.txt")
            .map_err(|e| anyhow!("Failed to read spacetime script: {}", e))?;

    let wallet_unspent_tree = db.open_tree(sync::UNSPENT)?;
    for x in wallet_unspent_tree.iter() {
        let (input_ivec, owner_amount_datum_ivec) = x?;
        let input = hex::encode(input_ivec);
        let (owner_pubkey, value, datum_option) =
            <(Address, Value, Option<Datum>)>::decode(&mut &owner_amount_datum_ivec[..])?;

        let spacetime_script: PlutusScript = PlutusScript(
            hex::decode(spacetime_script_hex).expect("Failed to decode spacetime script"),
        );
        let spacetime_hash: PolicyId = compute_plutus_v2_script_hash(spacetime_script.clone());
        let spacetime_address: Address = Address(
            hex::decode("70".to_owned() + &hex::encode(spacetime_hash))
                .map_err(|e| anyhow!("Failed to decode spacetime address: {}", e))?,
        );

        if owner_pubkey == spacetime_address {
            let ship_datum = datum_option.map(|d| ShipDatum::from(d));
            match ship_datum {
                None | Some(ShipDatum::MalformedShipDatum) => {
                    println!(
                        "{}: datum {:?}, value: {}",
                        input,
                        ship_datum,
                        value.normalize(),
                    );
                }
                Some(ShipDatum::Ok {
                    pos_x,
                    pos_y,
                    ship_token_name,
                    pilot_token_name,
                    last_move_latest_time,
                }) => {
                    println!(
                        "{}:\n {} {:?}\n {} {:?}\n {} {:?}\n {} {:?}\n {} {:?}\n {} {}\n",
                        input.bold(),
                        "POS X:".bold(),
                        pos_x,
                        "POS Y:".bold(),
                        pos_y,
                        "SHIP TOKEN NAME:".bold(),
                        ship_token_name.0,
                        "PILOT TOKEN NAME:".bold(),
                        pilot_token_name.0,
                        "LAST MOVE LATEST TIME:".bold(),
                        last_move_latest_time,
                        "VALUE:".bold(),
                        value.normalize(),
                    );
                }
            }
        }
    }

    Ok(())
}
