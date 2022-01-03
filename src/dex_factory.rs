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
)
// -> Vec<DexMarket<'a, M>>
where
    M: Middleware,
{
    let markets: Vec<DexMarket<'a, M>> = factory_addresses
        .into_iter()
        .map(|address| DexMarket::new(address, flash_query_contract, client.clone()))
        .collect();

    let weth_address = &WETH_ADDRESS.parse::<Address>().unwrap();

    let mut pairs: Vec<[H160; 3]> = vec![];
    for market in markets {
        let market_pairs = market
            .get_markets()
            .await
            .unwrap()
            .into_iter()
            .map(|pair| pair)
            .collect::<Vec<[H160; 3]>>();

        for pair in market_pairs {
            pairs.push(pair.to_owned());
        }
    }

    let pairs = pairs
        .into_iter()
        .filter(|pair| {
            // println!("pair[0]: {}", pair[0]);
            let is_include_weth = pair[0].eq(weth_address) || pair[1].eq(weth_address);
            is_include_weth
        })
        .sorted_by(|pair0, pair1| {
            let non_weth0 = if pair0[0].eq(weth_address) {
                pair0[1]
            } else {
                pair0[0]
            };
            let non_weth1 = if pair1[0].eq(weth_address) {
                pair1[1]
            } else {
                pair1[0]
            };
            non_weth1.cmp(&non_weth0)
        })
        .group_by(|pair| {
            if pair[0].eq(weth_address) {
                pair[1]
            } else {
                pair[0]
            }
        })
        .into_iter()
        .map(|(key, group)| {
            let pairs = group
                .map(|[_, _, pair_address]| pair_address)
                .collect::<Vec<H160>>();
            (key, pairs)
        })
        .filter(|(_, pairs)| pairs.len() > (1 as usize))
        .collect::<Vec<(H160, Vec<H160>)>>();

    dbg!(pairs);
    // for (key, vals) in &pairs.into_iter().group_by(|pair| {
    //     if pair[0].eq(weth_address) {
    //         pair[1]
    //     } else {
    //         pair[0]
    //     }
    // }) {
    //     let count = vals.count();
    //     if count > (2 as usize) {
    //         dbg!(key);
    //         dbg!(count);
    //         println!("------------------------------");
    //     }
    // }
    ()
    // todo!()
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
