use std::sync::Arc;

use crate::bindings::i_uniswap_v2_pair::IUniswapV2Pair;
use ethers::prelude::*;

pub struct UniswapPair<M> {
    contract: IUniswapV2Pair<M>,
}

impl<M> UniswapPair<M>
where
    M: Middleware,
{
    pub fn new(pair_address: &str, client: Arc<M>) -> Self {
        let address = pair_address.parse::<Address>().expect("Invalid Address");
        let contract = IUniswapV2Pair::new(address, client);
        Self { contract }
    }

    pub async fn updateReserve(&self) -> Result<(u128, u128, u32), ContractError<M>> {
        let reserves = self.contract.get_reserves().call().await;
        reserves
    }
}
