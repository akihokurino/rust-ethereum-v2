use crate::aws::lambda;
use crate::error::CliResult;
use crate::ethereum::ethers_rs::{hello, sample_oracle};
use crate::ethereum::rust_web3::{rust_token1155, rust_token721};
use crate::ethereum::{ethers_rs, rust_web3};
use crate::model::Schema;
use crate::CliError;
use std::fs::File;
use std::io::Read;

pub async fn send_eth(ether: f64, address: String) -> CliResult<()> {
    println!("{}", "use ethers-rs");
    ethers_rs::send_eth(
        ether,
        address.clone().parse::<ethers::prelude::Address>().unwrap(),
    )
    .await?;

    println!("{}", "use rust-web3");
    rust_web3::send_eth(ether, rust_web3::parse_address(address.clone()).unwrap()).await?;

    Ok(())
}

pub async fn mint_erc721(
    name: String,
    description: String,
    image_filename: String,
) -> CliResult<()> {
    if name.is_empty() || description.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_filename.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let mut file = File::open(format!("asset/{}", image_filename))?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    println!("{}", "uploading ipfs..........");
    let output = lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::create_metadata(
        &name,
        &description,
        "",
        base64::encode(buf),
    ))
    .await?;
    if output.ipfs_response.is_none() {
        return Err(CliError::Internal(
            "IPFSのサーバーが起動していません".to_string(),
        ));
    }
    let res = output.ipfs_response.unwrap();
    let ipfs_hash = res.hash;
    println!("ipfs_hash: {}", ipfs_hash.clone());
    println!("ipfs_url: {}", res.url);

    println!("{}", "minting..........");
    let erc721_cli = rust_token721::Client::new();
    erc721_cli.mint(ipfs_hash).await?;

    Ok(())
}

pub async fn mint_erc1155(
    name: String,
    description: String,
    image_filename: String,
    amount: u128,
) -> CliResult<()> {
    if name.is_empty() || description.is_empty() || amount <= 0 {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_filename.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let mut file = File::open(format!("asset/{}", image_filename))?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    println!("{}", "uploading ipfs..........");
    let output = lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::create_metadata(
        &name,
        &description,
        "",
        base64::encode(buf),
    ))
    .await?;
    if output.ipfs_response.is_none() {
        return Err(CliError::Internal(
            "IPFSのサーバーが起動していません".to_string(),
        ));
    }
    let res = output.ipfs_response.unwrap();
    let ipfs_hash = res.hash;
    println!("ipfs_hash: {}", ipfs_hash.clone());
    println!("ipfs_url: {}", res.url);

    println!("{}", "minting..........");
    let erc1155_cli = rust_token1155::Client::new();
    erc1155_cli.mint(ipfs_hash, amount).await?;

    Ok(())
}

pub async fn sell(token_id: String, schema: Schema, ether: f64) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::sell(
        &schema.address(),
        &token_id,
        &schema,
        ether,
    ))
    .await?;

    Ok(())
}

pub async fn transfer(token_id: String, schema: Schema, to_address: String) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::transfer(
        &schema.address(),
        &token_id,
        &schema,
        &to_address,
    ))
    .await?;

    Ok(())
}

pub async fn create_get_time_request() -> CliResult<()> {
    let cli = sample_oracle::Client::new();
    cli.create_get_time_request().await?;

    Ok(())
}

pub async fn set_hello_message(message: String) -> CliResult<()> {
    let cli = hello::Client::new();
    cli.set_message(message).await?;

    Ok(())
}
