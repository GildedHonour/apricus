use anyhow::Result;
mod eth_wallet;
mod utils;
use std::env;
use std::str::FromStr;
use web3::types::Address;
use std::path::Path;

const WALLET_FILE_PATH: &str = "wallet_data.json";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let (secret_key, pub_key) = eth_wallet::generate_keypair();
    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);

    if !Path::new(WALLET_FILE_PATH).exists() {
        let wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
        wallet.save_to_file(WALLET_FILE_PATH)?;
    }

    let loaded_wallet = eth_wallet::Wallet::from_file(WALLET_FILE_PATH)?;
    println!("wallet: {:?}", loaded_wallet);

    let endpoint = env::var("INFURA_WSS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("balance: {} eth", &balance);

    Ok(())
}
