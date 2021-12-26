use ethers::{
    providers::{Http, Provider},
    signers::{LocalWallet},
    middleware::SignerMiddleware,
    core::types::{Address}
};

use dotenv::dotenv;
mod bindings;
use crate::bindings::*;

use std::{sync::Arc};

#[tokio::main]
async fn main() {
		println!("hello");
    dotenv().ok();

    let pk = dotenv::var("PRIVATE_KEY").unwrap();
    let wallet: LocalWallet = pk.parse().expect("fail parse");
    // println!("{}", pk);
    let provider_id = dotenv::var("PROVIDER_ID").unwrap();
    let url = format!("https://mainnet.infura.io/v3/{}", provider_id);
    

    let provider_service = Provider::<Http>::try_from(url).expect("failed");

    let provider = Arc::new(SignerMiddleware::new(provider_service, wallet));

    
    let address = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc".parse::<Address>().expect("Invalid Address");
    let pair = i_uniswap_v2_pair::IUniswapV2Pair::new(address, provider);

    let token0 = pair.token_0();
    let token0 = token0.call();
    let token0 = token0.await.unwrap();
    println!("token0: {:?}", token0);
}