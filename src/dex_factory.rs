use itertools::Itertools;
use std::sync::Arc;

use crate::{
    addresses::WETH_ADDRESS,
    bindings::{
        flash_bots_uniswap_query::FlashBotsUniswapQuery, i_uniswap_v2_factory::IUniswapV2Factory,
    },
};
use ethers::prelude::*;

pub async fn get_markets_by_token<'a, M>(
    factory_addresses: Vec<Address>,
    flash_query_contract: &'a FlashBotsUniswapQuery<M>,
    client: Arc<M>,
) -> Vec<DexMarket<'a, M>>
where
    M: Middleware,
{
    let markets: Vec<DexMarket<'a, M>> = factory_addresses
        .into_iter()
        .map(|address| DexMarket::new(address, flash_query_contract, client.clone()))
        .collect();

    let mut pairs: Vec<([H160; 3], [H160; 3], [H160; 3])> = vec![];
    for market in &markets {
        let market_pair = &market.get_markets().await.unwrap();
        let token0 = market_pair[0];
        let token1 = market_pair[1];
        let pair_address = market_pair[2];
        pairs.push((token0, token1, pair_address));
    }

    for (key, vals) in &pairs.into_iter().group_by(|pair| {
        if *pair.0[0].to_string() == *WETH_ADDRESS {
            pair.0[0].to_string()
        } else {
            pair.0[1].to_string()
        }
    }) {
        dbg!(key);
        dbg!(vals.count());
        println!("------------------------------");
    }

    // markets.into_iter().group_by(|market| {
    //     let market_pair = &market.get_markets().await.unwrap();
    // });

    markets
}

pub struct DexMarket<'a, M> {
    factory_contract: IUniswapV2Factory<M>,
    flash_query_contract: &'a FlashBotsUniswapQuery<M>,
}

// This should hold Factory of each Dex
// and have a method to query pairs
impl<'a, M> DexMarket<'a, M>
where
    M: Middleware,
{
    pub fn new(
        pair_address: Address,
        flash_query_contract: &'a FlashBotsUniswapQuery<M>,
        client: Arc<M>,
    ) -> Self {
        // let pair_address = pair_address.parse::<Address>().expect("Invalid Address");
        // let flash_query_address = flash_query_address
        //     .parse::<Address>()
        //     .expect("Invalid Address");
        let contract = IUniswapV2Factory::new(pair_address, client.clone());
        Self {
            factory_contract: contract,
            flash_query_contract,
        }
    }

    pub async fn get_markets(&self) -> Result<Vec<[H160; 3]>, ContractError<M>> {
        let start = U256::from_dec_str("0").unwrap();
        let stop = U256::from_dec_str("1000").unwrap();

        let pairs = self
            .flash_query_contract
            .get_pairs_by_index_range(self.factory_contract.address(), start, stop)
            .call()
            .await;
        pairs
    }
}
