use std::sync::Arc;

use crate::bindings::i_uniswap_v2_pair::IUniswapV2Pair;
use ethers::prelude::*;

#[derive(Debug)]
pub struct UniswapPair<M>
where
    M: Middleware,
{
    contract: IUniswapV2Pair<M>,
}

impl<M> UniswapPair<M>
where
    M: Middleware,
{
    pub fn new(pair_address: Address, client: Arc<M>) -> Self {
        // let address = pair_address.parse::<Address>().expect("Invalid Address");
        let contract = IUniswapV2Pair::new(pair_address, client);
        Self { contract }
    }

    pub async fn debug(&self) {}

    pub async fn update_reserve(&self) -> Result<(u128, u128, u32), ContractError<M>> {
        let reserves = self.contract.get_reserves().call().await;
        reserves
    }
}
