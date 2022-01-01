use std::sync::Arc;

use ethers::{
    middleware::SignerMiddleware,
    prelude::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

use dotenv::dotenv;
use forge_test::uniswap::UniswapPair;
use futures::StreamExt;

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

    let pair = UniswapPair::new(
        "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc",
        Arc::new(client),
    );

    let fut = provider_service.watch_blocks();
    let mut stream = fut.await.unwrap().take(5);
    while let Some(block) = stream.next().await {
        let reserves = pair.updateReserve().await;
        dbg!(block);
        let blocknumber = provider_service.get_block_number();
        let number = blocknumber.await.unwrap();
        dbg!(number);
        dbg!(reserves.expect("cant get reserve"));
    }
}
