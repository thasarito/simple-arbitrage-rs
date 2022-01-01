use std::sync::Arc;

// use ethers::prelude::*;
use ethers::{
    core::types::Address,
    middleware::SignerMiddleware,
    prelude::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

use dotenv::dotenv;
use futures::StreamExt;
mod bindings;
use crate::bindings::*;

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

    let client = SignerMiddleware::new(provider_service.clone(), wallet);

    let address = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc"
        .parse::<Address>()
        .expect("Invalid Address");
    let pair = i_uniswap_v2_pair::IUniswapV2Pair::new(address, Arc::new(client));

    let fut = provider_service.watch_blocks();
    let mut stream = fut.await.unwrap().take(5);
    while let Some(block) = stream.next().await {
        dbg!(block);
        let blocknumber = provider_service.get_block_number();
        let number = blocknumber.await.unwrap();
        dbg!(number);
    }

    let token0 = pair.token_0();
    let token0 = token0.call();
    let token0 = token0.await.unwrap();
    println!("token0: {:?}", token0);
}
