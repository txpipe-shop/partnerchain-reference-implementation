use crate::cli::{RotateCommitteeArgs, ShowOutputsWithAssetArgs};
use crate::{rpc, sync};
use anyhow::anyhow;
use griffin_core::checks_interface::{babbage_minted_tx_from_cbor, babbage_tx_to_cbor};
use griffin_core::pallas_codec::utils::MaybeIndefArray::{Def, Indef};
use griffin_core::pallas_primitives::babbage::{
    Constr, MintedTx, PlutusData as PallasPlutusData, Tx as PallasTransaction,
};
use griffin_core::pallas_traverse::OriginalHash;
use griffin_core::types::{
    compute_plutus_v2_script_hash, Address, Input, Multiasset, Output, PlutusData, PlutusScript,
    PolicyId, Redeemer, RedeemerTag, Transaction, VKeyWitness, Value,
};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::rpc_params;
use parity_scale_codec::Encode;
use sc_keystore::LocalKeystore;
use sled::Db;
use sp_core::ed25519::Public;
use sp_runtime::traits::{BlakeTwo256, Hash};

const ROTATE_SCRIPT: &str = "";

/// Builds the TX that consumes the next_committee, creating a new current_committe
/// with the corresponding datum.
///
/// TODO
/// - add error management
/// - add checks for datum
pub async fn build_tx(
    db: &Db,
    client: &HttpClient,
    keystore: &LocalKeystore,
    args: RotateCommitteeArgs,
) -> anyhow::Result<()> {
    let Some(cmt) = rpc::node_get_committee_data(client).await.unwrap() else {
        log::info!("Failed to obtain committee data from node");
        return Err(anyhow!("Failed to obtain committee data from Node"));
    };
    let committee_rotation_script: PlutusScript =
        PlutusScript(hex::decode(ROTATE_SCRIPT).expect("Failed to decode committee_rotation script"));
    let committee_rotation_hash: PolicyId =
        compute_plutus_v2_script_hash(committee_rotation_script.clone());
    let committee_rotation_address: Address = Address(
        hex::decode("70".to_owned() + &hex::encode(committee_rotation_hash))
            .map_err(|e| anyhow!("Failed to decode committee_rotation address: {}", e))?,
    );

    assert_eq!(
        committee_rotation_address, cmt.address,
        "committee rotation script address mismatch"
    );
    assert_eq!(
        committee_rotation_hash, cmt.policy_id,
        "committee rotation policy id mismatch"
    );

    let [ref next_input] = sync::get_outputs_with_asset(
        db,
        ShowOutputsWithAssetArgs {
            policy: cmt.policy_id,
            name: cmt.next_asset_name.clone().0,
        },
    )
    .unwrap()[..] else {
        log::info!("Next committee utxo doesn't exist, can't rotate");
        return Err(anyhow!("Next committee utxo doesn't exist, can't rotate"));
    };

    if next_input.address != cmt.address {
        log::info!("Next committee utxo doesn't belong to expected address");
        return Err(anyhow!("Next_cmt utxo doesnt belong to expected address"));
    }

    assert!(
        next_input.datum_option.is_some(),
        "datum in next cmt utxo shouldnt be None"
    );

    let [ref current_input] = sync::get_outputs_with_asset(
        db,
        ShowOutputsWithAssetArgs {
            policy: cmt.policy_id,
            name: cmt.current_asset_name.0,
        },
    )
    .unwrap()[..] else {
        log::info!("Current committee utxo doesn't exist");
        return Err(anyhow!("No Utxo for current committee"));
    };

    if current_input.address != cmt.address {
        log::info!("Current committee utxo doesn't belong to expected address");
        return Err(anyhow!(
            "current_cmt utxo doesnt belong to expected address"
        ));
    }

    let Some((owner_pubkey, owner_value, _)) = sync::get_unspent(db, &args.input)? else {
        return Err(anyhow!("Input not found"));
    };

    let mut transaction = Transaction::from((Vec::new(), Vec::new()));
    transaction.transaction_body.inputs = vec![
        next_input.input.clone(),
        current_input.input.clone(),
        args.input,
    ];

    let outputs = vec![
        Output {
            address: current_input.address.clone(),
            value: current_input.value.clone(),
            datum_option: next_input.datum_option.clone(),
        },
        Output {
            address: owner_pubkey,
            value: owner_value + Value::Coin(next_input.value.coin_of()),
            datum_option: None,
        },
    ];

    for output in outputs {
        transaction.transaction_body.outputs.push(output.clone());
    }

    let mint = Some(Multiasset::from((
        cmt.policy_id,
        cmt.next_asset_name.clone(),
        -1,
    )));

    transaction.transaction_body.mint = mint;
    transaction.transaction_body.validity_interval_start = Some(args.validity_interval_start);
    transaction.transaction_body.ttl = Some(args.ttl);

    let pallas_tx: PallasTransaction = <_>::from(transaction.clone());
    let cbor_bytes: Vec<u8> = babbage_tx_to_cbor(&pallas_tx);
    let mtx: MintedTx = babbage_minted_tx_from_cbor(&cbor_bytes);
    let tx_hash: &Vec<u8> = &Vec::from(mtx.transaction_body.original_hash().as_ref());

    let vkey: Vec<u8> = Vec::from(args.witness.0);
    let public = Public::from_h256(args.witness);
    let signature: Vec<u8> = Vec::from(
        crate::keystore::sign_with(keystore, &public, tx_hash)
            .map_err(|e| {
                log::info!("Failed to sign transaction: {}", e);
                anyhow!("Failed to sign transaction: {}", e)
            })?
            .0,
    );
    transaction.transaction_witness_set = <_>::from(vec![VKeyWitness::from((vkey, signature))]);

    let redeemer_current = Redeemer {
        tag: RedeemerTag::Spend,
        index: 2,
        data: PlutusData::from(PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef(vec![]),
        })),
    };
    let redeemer_next = Redeemer {
        tag: RedeemerTag::Spend,
        index: 1,
        data: PlutusData::from(PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Def(vec![]),
        })),
    };
    let next_mint_redeemer = Redeemer {
        tag: RedeemerTag::Mint,
        index: 0,
        data: PlutusData::from(PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Def(vec![]),
        })),
    };

    transaction.transaction_witness_set.plutus_script = Some(vec![committee_rotation_script]);
    transaction.transaction_witness_set.redeemer =
        Some(vec![redeemer_next, redeemer_current, next_mint_redeemer]);

    log::debug!("Griffin transaction is: {:#x?}", transaction);

    let pallas_tx: PallasTransaction = <_>::from(transaction.clone());
    log::info!("Babbage transaction is: {:#x?}", pallas_tx);
    log::debug!(
        "Babbage transaction encoded is: {:?}",
        hex::encode(babbage_tx_to_cbor(&pallas_tx))
    );

    // Send the transaction
    let genesis_spend_hex = hex::encode(Encode::encode(&transaction));

    log::info!("CBOR HEX TRANSACTION: {:?}", genesis_spend_hex);

    let params = rpc_params![genesis_spend_hex];
    let genesis_spend_response: Result<String, _> =
        client.request("author_submitExtrinsic", params).await;
    log::info!(
        "Node's response to spend transaction: {:?}",
        genesis_spend_response
    );
    if let Err(_) = genesis_spend_response {
        Err(anyhow!("Node did not accept the transaction"))?;
    } else {
        println!("Transaction queued. When accepted, the following UTxOs will become available:");
        // Print new output refs for user to check later
        let tx_hash = <BlakeTwo256 as Hash>::hash_of(&Encode::encode(&transaction));
        for (i, output) in transaction.transaction_body.outputs.iter().enumerate() {
            let new_value_ref = Input {
                tx_hash,
                index: i as u32,
            };
            let amount = &output.value;

            println!(
                "{:?} worth {amount:?}.",
                hex::encode(Encode::encode(&new_value_ref))
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use griffin_core::{
        pallas_codec::{minicbor, utils::Int},
        pallas_primitives::BoundedBytes,
    };

    #[test]
    fn test_phase2_pay_true() {
        use super::ROTATE_SCRIPT;
        use griffin_core::checks_interface::conway_minted_tx_from_cbor;
        use griffin_core::pallas_codec::utils::MaybeIndefArray::{Def, Indef};
        use griffin_core::pallas_primitives::{
            babbage::Tx as BabbageTx,
            conway::{
                BigInt, Constr, MintedTx as ConwayMintedTx, PlutusData as PallasPlutusData,
                TransactionInput, TransactionOutput,
            },
        };
        use griffin_core::types::{
            compute_plutus_v2_script_hash, Address, AssetName, Multiasset, Output, PlutusData,
            PlutusScript, Redeemer, RedeemerTag, Value,
        };
        use griffin_core::uplc::tx::SlotConfig;
        use griffin_core::uplc::tx::{eval_phase_two, ResolvedInput};
        use griffin_core::{
            checks_interface::babbage_tx_to_cbor,
            types::{Datum, Input, Transaction},
        };
        use parity_scale_codec::Decode;
        use sp_core::H256;

        env_logger::init();

        log::info!("ARRANCO ");

        const SLOT_CONFIG: SlotConfig = SlotConfig {
            zero_time: 1747081100000,
            zero_slot: 0,
            slot_length: 3000,
        };

        // PARSE SCRIPTS
        let rotate_script = PlutusScript(hex::decode(ROTATE_SCRIPT).unwrap());
        let rotate_policy = compute_plutus_v2_script_hash(rotate_script.clone());

        // Asset Names
        let next_name = AssetName::from("NextAuthorities".to_string());

        let next_redeemer = Redeemer {
            tag: RedeemerTag::Spend,
            index: 0,
            data: PlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Def([].to_vec()),
            })),
        };

        let id = hex::decode("022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC")
            .unwrap();
        let aura =
            hex::decode("022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC")
                .unwrap();
        let grandpa =
            hex::decode("022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC")
                .unwrap();

        // let next_datum = PallasPlutusData::from(
        //     PallasPlutusData::Constr(Constr {
        //             tag: 121,
        //             any_constructor: None,
        //             fields: Indef(
        //                 [
        //                     PallasPlutusData::BoundedBytes(BoundedBytes(id)),
        //                     PallasPlutusData::Constr(Constr {
        //                         tag: 121,
        //                         any_constructor: None,
        //                         fields: Indef(
        //                             [
        //                                 PallasPlutusData::BoundedBytes(BoundedBytes(
        //                                     aura,
        //                                 )),
        //                                 PallasPlutusData::BoundedBytes(BoundedBytes(
        //                                     grandpa,
        //                                 )),
        //                                 PallasPlutusData::BigInt(BigInt::Int(Int(
        //                                     minicbor::data::Int::from(1),
        //                                 ))),
        //                             ]
        //                             .to_vec(),
        //                         ),
        //                     }),
        //                 ]
        //                 .to_vec(),
        //             ),
        //         })
        // );

        let next_datum = PallasPlutusData::from(PallasPlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: Indef([].to_vec()),
        }));

        let next_input_tx_id = "b18283b185f7d184c5f205d2abac5c80d07a0a965bb696f93273db16527b8845";
        let next_input_index = 5;

        let next_input = Input {
            tx_hash: H256::from_slice(hex::decode(next_input_tx_id).unwrap().as_slice()),
            index: next_input_index,
        };

        let inputs = vec![next_input];
        let resolved_inputs = vec![Output {
            address: Address(hex::decode("70".to_owned() + &hex::encode(rotate_policy)).unwrap()),
            value: Value::Coin(314150000),
            datum_option: Some(Datum(PlutusData::from(next_datum).0)),
        }];

        let pallas_inputs = inputs
            .iter()
            .map(|i| TransactionInput::from(i.clone()))
            .collect::<Vec<_>>();
        let pallas_resolved_inputs = resolved_inputs
            .iter()
            .map(|ri| TransactionOutput::from(ri.clone()))
            .collect::<Vec<_>>();

        let mut transaction = Transaction::from((Vec::new(), Vec::new()));
        for input in inputs {
            transaction.transaction_body.inputs.push(input.clone());
        }

        let outputs = vec![Output {
            address: Address(
                hex::decode(
                    "70".to_owned() + ("01e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4"),
                )
                .unwrap(),
            ),
            value: Value::Coin(314150000),
            datum_option: None,
        }];

        for output in outputs {
            transaction.transaction_body.outputs.push(output.clone());
        }

        transaction.transaction_body.validity_interval_start = Some(1606059091000);
        transaction.transaction_witness_set.redeemer = Some(vec![next_redeemer]);
        transaction.transaction_witness_set.plutus_script = Some(vec![rotate_script]);

        let pallas_tx: BabbageTx = <_>::from(transaction.clone());
        let cbor_bytes: Vec<u8> = babbage_tx_to_cbor(&pallas_tx);
        let mtx: ConwayMintedTx = conway_minted_tx_from_cbor(&cbor_bytes);

        let input_utxos: Vec<ResolvedInput> = pallas_inputs
            .iter()
            .zip(pallas_resolved_inputs.iter())
            .map(|(input, output)| ResolvedInput {
                input: input.clone(),
                output: output.clone(),
            })
            .collect();
        log::info!("LLego");
        let redeemers =
            eval_phase_two(&mtx, &input_utxos, None, None, &SLOT_CONFIG, false, |_| ()).unwrap();
        assert_eq!(redeemers.len(), 1);
    }

    #[test]
    fn test_phase2_rotate_cmt() {
        use super::ROTATE_SCRIPT;
        use griffin_core::checks_interface::conway_minted_tx_from_cbor;
        use griffin_core::pallas_codec::utils::MaybeIndefArray::Indef;
        use griffin_core::pallas_primitives::{
            babbage::Tx as BabbageTx,
            conway::{
                BigInt, Constr, MintedTx as ConwayMintedTx, PlutusData as PallasPlutusData,
                TransactionInput, TransactionOutput,
            },
        };
        use griffin_core::types::{
            compute_plutus_v2_script_hash, Address, AssetName, Multiasset, Output, PlutusData,
            PlutusScript, Redeemer, RedeemerTag, Value,
        };
        use griffin_core::uplc::tx::SlotConfig;
        use griffin_core::uplc::tx::{eval_phase_two, ResolvedInput};
        use griffin_core::{
            checks_interface::babbage_tx_to_cbor,
            types::{Datum, Input, Transaction},
        };
        use parity_scale_codec::Decode;
        use sp_core::H256;
        log::info!("ARRANCO ");

        const SLOT_CONFIG: SlotConfig = SlotConfig {
            zero_time: 1747081100000,
            zero_slot: 0,
            slot_length: 3000,
        };

        // PARSE SCRIPTS
        let rotate_script = PlutusScript(hex::decode(ROTATE_SCRIPT).unwrap());
        let rotate_policy = compute_plutus_v2_script_hash(rotate_script.clone());

        // Asset Names
        let next_name = AssetName::from("NextAuthorities".to_string());
        let current_name = AssetName::from("Authorities".to_string());

        // BURNS
        let burn_next = Some(Multiasset::from((rotate_policy, next_name.clone(), -1)));

        let next_redeemer = Redeemer {
            tag: RedeemerTag::Spend,
            index: 1,
            data: PlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [PallasPlutusData::Constr(Constr {
                        tag: 122,
                        any_constructor: None,
                        fields: Indef([].to_vec()),
                    })]
                    .to_vec(),
                ),
            })),
        };
        let current_redeemer = Redeemer {
            tag: RedeemerTag::Spend,
            index: 0,
            data: PlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef(
                    [PallasPlutusData::Constr(Constr {
                        tag: 122,
                        any_constructor: None,
                        fields: Indef([].to_vec()),
                    })]
                    .to_vec(),
                ),
            })),
        };
        let next_burn_redeemer = Redeemer {
            tag: RedeemerTag::Mint,
            index: 0,
            data: PlutusData::from(PallasPlutusData::Constr(Constr {
                tag: 121,
                any_constructor: None,
                fields: Indef([].to_vec()),
            })),
        };

        let current_datum_raw = hex::decode("D8799F9FD8799F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BACD8799F5820D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D582088DC3417D5058EC4B4503E0C12EA1A0A89BE200FE98922423D4334014FA6B0EE01FFFFFFFF").unwrap();

        let next_datum_raw = hex::decode("D8799F9FD8799F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BACD8799F5820D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D582088DC3417D5058EC4B4503E0C12EA1A0A89BE200FE98922423D4334014FA6B0EE01FFFFD8799F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BACD8799F58208EAF04151687736326C9FEA17E25FC5287613693C912909CB226AA4794F26A485820D17C2D7823EBF260FD138F2D7E27D114C0145D968B5FF5006125F2414FADAE6901FFFFFFFF").unwrap();

        // BUILD DATUMS
        let _current_datum = PlutusData::decode(&mut &current_datum_raw[..]).unwrap();

        let _next_datum = PlutusData::decode(&mut &next_datum_raw[..]).unwrap();

        // BUILD INPUTS
        let current_input_tx_id =
            "b18283b185f7d184c5f205d2abac5c80d07a0b965bb696f93273db16527b8845";
        let current_input_index = 4;

        let next_input_tx_id = "b18283b185f7d184c5f205d2abac5c80d07a0b965bb696f93273db16527b8845";
        let next_input_index = 5;

        let current_input = Input {
            tx_hash: H256::from_slice(hex::decode(current_input_tx_id).unwrap().as_slice()),
            index: current_input_index,
        };
        let next_input = Input {
            tx_hash: H256::from_slice(hex::decode(next_input_tx_id).unwrap().as_slice()),
            index: next_input_index,
        };

        let inputs = vec![current_input, next_input];
        let resolved_inputs = vec![
            Output {
                address: Address(
                    hex::decode("70".to_owned() + &hex::encode(rotate_policy)).unwrap(),
                ),
                value: Value::Coin(314150000)
                    + Value::from((1, rotate_policy, current_name.clone(), 1)),
                datum_option: Some(Datum(current_datum_raw)),
            },
            Output {
                address: Address(
                    hex::decode("70".to_owned() + &hex::encode(rotate_policy)).unwrap(),
                ),
                value: Value::Coin(314150000)
                    + Value::from((1, rotate_policy, next_name.clone(), 1)),
                datum_option: Some(Datum(next_datum_raw.clone())),
            },
        ];

        let pallas_inputs = inputs
            .iter()
            .map(|i| TransactionInput::from(i.clone()))
            .collect::<Vec<_>>();
        let pallas_resolved_inputs = resolved_inputs
            .iter()
            .map(|ri| TransactionOutput::from(ri.clone()))
            .collect::<Vec<_>>();

        let mut transaction = Transaction::from((Vec::new(), Vec::new()));
        for input in inputs {
            transaction.transaction_body.inputs.push(input.clone());
        }

        let outputs = vec![Output {
            address: Address(hex::decode("70".to_owned() + &hex::encode(rotate_policy)).unwrap()),
            value: Value::Coin(314150000)
                + Value::from((1, rotate_policy, current_name.clone(), 1)),
            datum_option: Some(Datum(next_datum_raw)),
        }];

        for output in outputs {
            transaction.transaction_body.outputs.push(output.clone());
        }

        transaction.transaction_body.mint = burn_next;
        transaction.transaction_body.validity_interval_start = Some(1606059091000);
        transaction.transaction_witness_set.redeemer =
            Some(vec![current_redeemer, next_redeemer, next_burn_redeemer]);
        transaction.transaction_witness_set.plutus_script = Some(vec![rotate_script]);

        let pallas_tx: BabbageTx = <_>::from(transaction.clone());
        let cbor_bytes: Vec<u8> = babbage_tx_to_cbor(&pallas_tx);
        let mtx: ConwayMintedTx = conway_minted_tx_from_cbor(&cbor_bytes);

        let input_utxos: Vec<ResolvedInput> = pallas_inputs
            .iter()
            .zip(pallas_resolved_inputs.iter())
            .map(|(input, output)| ResolvedInput {
                input: input.clone(),
                output: output.clone(),
            })
            .collect();
        log::info!("LLego");
        let redeemers =
            eval_phase_two(&mtx, &input_utxos, None, None, &SLOT_CONFIG, false, |_| ()).unwrap();
        assert_eq!(redeemers.len(), 4);
    }
}
