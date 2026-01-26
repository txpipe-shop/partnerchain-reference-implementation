use crate::{
    PermissionedCandidatesArgs, UtxosByAddressArgs, UtxosByAddressAssetArgs, DOLOS_ENDPOINT,
};
use blockfrost::{BlockFrostSettings, BlockfrostAPI, Pagination};
use hex::encode;
use reqwest::ClientBuilder;

pub async fn utxos_by_address(args: UtxosByAddressArgs) -> anyhow::Result<()> {
    let mut blockfrost_settings = BlockFrostSettings::new();

    blockfrost_settings.base_url = Some(DOLOS_ENDPOINT.to_string());
    let api = BlockfrostAPI::new_with_client("", blockfrost_settings, ClientBuilder::new())?;

    let pagination = Pagination::default();
    let utxos = api.addresses_utxos(&args.address, pagination).await?;
    println!("{:#?}", utxos);

    Ok(())
}

pub async fn utxos_by_address_asset(args: UtxosByAddressAssetArgs) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let request = client.get(format!(
        "{}/addresses/{}/utxos/{}",
        DOLOS_ENDPOINT, args.address, args.asset
    ));

    let response = request.send().await?.text().await?;
    println!("{:#?}", response);

    Ok(())
}

pub async fn permissioned_candidates(args: PermissionedCandidatesArgs) -> anyhow::Result<()> {
    let mut blockfrost_settings = BlockFrostSettings::new();

    blockfrost_settings.base_url = Some(DOLOS_ENDPOINT.to_string());
    let api = BlockfrostAPI::new_with_client("", blockfrost_settings, ClientBuilder::new())?;

    let pagination = Pagination::default();
    let utxos = api.addresses_utxos(&args.address, pagination).await?;
    println!("{:#?}", utxos);

    let candidates_utxo = utxos
        .first()
        .ok_or_else(|| anyhow::anyhow!("No UTxO found for address {}", args.address))?;

    let datum_hex = candidates_utxo
        .inline_datum
        .clone()
        .expect("No inline datum found in UTxO");

    let datum_bytes = hex::decode(datum_hex)?;
    let permissioned_candidates = demo_authorities::parse_authorities(&datum_bytes)
        .expect("Failed to parse authorities datum");
    println!(
        "Permissioned Candidates Aura Keys: {:#?}",
        permissioned_candidates
            .0
            .iter()
            .map(|b| encode(Vec::<u8>::from(b.clone())))
            .collect::<Vec<String>>()
    );
    println!(
        "Permissioned Candidates Grandpa Keys: {:#?}",
        permissioned_candidates
            .1
            .iter()
            .map(|b| encode(Vec::<u8>::from(b.clone())))
            .collect::<Vec<String>>()
    );

    Ok(())
}
