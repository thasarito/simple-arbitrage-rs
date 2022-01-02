use std::sync::Arc;

use crate::bindings::{
    flash_bots_uniswap_query::FlashBotsUniswapQuery, i_uniswap_v2_factory::IUniswapV2Factory,
};
use ethers::prelude::*;

pub struct DexMarket<M> {
    contract: IUniswapV2Factory<M>,
    flash_query_contract: FlashBotsUniswapQuery<M>,
}

impl<M> DexMarket<M>
where
    M: Middleware,
{
    pub fn new(pair_address: &str, flash_query_address: &str, client: Arc<M>) -> Self {
        let pair_address = pair_address.parse::<Address>().expect("Invalid Address");
        let flash_query_address = flash_query_address
            .parse::<Address>()
            .expect("Invalid Address");
        let contract = IUniswapV2Factory::new(pair_address, client.clone());
        let flash_query_contract = FlashBotsUniswapQuery::new(flash_query_address, client.clone());
        Self {
            contract,
            flash_query_contract,
        }
    }

    pub async fn debug(&self) {}

    pub async fn get_markets(&self) -> Result<Vec<[H160; 3]>, ContractError<M>> {
        let start = U256::from_dec_str("0").unwrap();
        let stop = U256::from_dec_str("1000").unwrap();

        let pairs = self
            .flash_query_contract
            .get_pairs_by_index_range(self.contract.address(), start, stop)
            .call()
            .await;
        pairs
    }
}
