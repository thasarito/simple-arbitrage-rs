use std::{ops::Add, sync::Arc};

use ethers::{
    abi::Address,
    middleware::SignerMiddleware,
    prelude::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

use dotenv::dotenv;
use forge_test::{
    addresses::*,
    // bindings::i_uniswap_v2_factory::IUniswapV2Factory,
    bindings::flash_bots_uniswap_query::FlashBotsUniswapQuery,
    dex_factory::get_markets_by_token,
    uniswap_pair::UniswapPair,
};
use futures::{future, StreamExt};

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

    let client = Arc::new(SignerMiddleware::new(provider_service.clone(), wallet));

    let factory_addresses = vec![
        SUSHISWAP_FACTORY_ADDRESS,
        UNISWAP_FACTORY_ADDRESS,
        ZEUS_FACTORY_ADDRESS,
        LUA_FACTORY_ADDRESS,
        CRO_FACTORY_ADDRESS,
    ]
    .into_iter()
    .map(|address| {
        address
            .parse::<Address>()
            .expect("parse factory address failed")
    })
    .collect::<Vec<Address>>();

    let flash_query_address = LOOKUP_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let flash_query_contract = FlashBotsUniswapQuery::new(flash_query_address, client.clone());
    let markets = get_markets_by_token(factory_addresses, &flash_query_contract, client.clone());
    // dbg!(markets);
    // let market = DexMarket::new(
    //     "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f",
    //     "0x5EF1009b9FCD4fec3094a5564047e190D72Bd511",
    //     client.clone(),
    // );
    // let allpairs = market.get_markets().await.unwrap();
    // let all_pairs = factory.all_pairs_length().call().await.unwrap();
    // dbg!(all_pairs);

    let pair = UniswapPair::new("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc", client.clone());

    let fut = provider_service.watch_blocks();
    let mut stream = fut.await.unwrap().take_while(|_| future::ready(true));
    while let Some(block) = stream.next().await {
        let reserves = pair.update_reserve().await.unwrap();
        dbg!(block);
        let blocknumber = provider_service.get_block_number();
        let number = blocknumber.await.unwrap();
        dbg!(number);
        dbg!(reserves);
    }
}
