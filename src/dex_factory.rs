use itertools::Itertools;
use std::sync::Arc;

use crate::{
    addresses::WETH_ADDRESS,
    bindings::{
        flash_bots_uniswap_query::FlashBotsUniswapQuery, i_uniswap_v2_factory::IUniswapV2Factory,
    },
};
use ethers::prelude::*;

pub async fn get_markets_by_token<M>(
    factory_addresses: Vec<Address>,
    flash_query_contract: &FlashBotsUniswapQuery<M>,
    client: Arc<M>,
) -> Vec<(H160, Vec<[H160; 3]>)>
where
    M: Middleware,
{
    let factories: Vec<DexFactory<M>> = factory_addresses
        .into_iter()
        .map(|address| DexFactory::new(address, flash_query_contract, client.clone()))
        .collect();

    let weth_address = &WETH_ADDRESS.parse::<Address>().unwrap();

    let mut pairs: Vec<[H160; 3]> = vec![];
    for factory in factories {
        let market_pairs = factory
            .get_markets()
            .await
            .into_iter()
            .collect::<Vec<[H160; 3]>>();

        for pair in market_pairs {
            pairs.push(pair.to_owned());
        }
    }

    let grouped_pairs: Vec<(H160, Vec<[H160; 3]>)> = pairs
        .into_iter()
        .filter(|pair| {
            // println!("pair[0]: {}", pair[0]);
            pair[0].eq(weth_address) || pair[1].eq(weth_address)
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
            let pairs = group.collect::<Vec<[H160; 3]>>();
            (key, pairs)
        })
        .filter(|(_, pairs)| pairs.len() > (1_usize))
        .collect::<Vec<(H160, Vec<[H160; 3]>)>>();

    grouped_pairs
}

pub struct DexFactory<'a, M> {
    factory_contract: IUniswapV2Factory<M>,
    flash_query_contract: &'a FlashBotsUniswapQuery<M>,
}

// This should hold Factory of each Dex
// and have a method to query pairs
impl<'a, M> DexFactory<'a, M>
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
        let contract = IUniswapV2Factory::new(pair_address, client);
        Self {
            factory_contract: contract,
            flash_query_contract,
        }
    }

    pub async fn get_markets(&self) -> Vec<[H160; 3]> {
        let batch_size = U256::from(1000u32);
        let mut start = U256::from(0u32);

        let batch_limit = 10;
        let mut count = 0;
        let mut markets: Vec<[H160; 3]> = vec![];
        loop {
            let stop = start + batch_size;

            let pairs = self
                .flash_query_contract
                .get_pairs_by_index_range(self.factory_contract.address(), start, stop)
                .call()
                .await
                .unwrap();

            count += 1;
            start = stop;

            let pair_length = pairs.len();

            markets.extend(pairs);

            if pair_length < batch_size.as_usize() || count > batch_limit {
                dbg!(markets.len());
                break;
            }
        }

        markets
    }
}
