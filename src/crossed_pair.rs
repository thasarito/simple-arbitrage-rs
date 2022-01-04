use crate::bindings::flash_bots_uniswap_query::FlashBotsUniswapQuery;
use ethers::prelude::*;

#[derive(Debug)]
pub struct CrossedPairManager<'a, M>
where
    M: Middleware,
{
    // contract: IUniswapV2Pair<M>,
    flash_query_contract: &'a FlashBotsUniswapQuery<M>,
    markets: Vec<TokenMarket<'a>>,
}

impl<'a, M> CrossedPairManager<'a, M>
where
    M: Middleware,
{
    pub fn new(
        grouped_pairs: &'a Vec<(H160, Vec<H160>)>,
        flash_query_contract: &'a FlashBotsUniswapQuery<M>,
    ) -> Self {
        let pairs = grouped_pairs
            .into_iter()
            .map(|(token, pairs)| TokenMarket {
                token,
                pairs: pairs
                    .to_vec()
                    .into_iter()
                    .map(|address| Pair {
                        address,
                        reserve0: None,
                        reserve1: None,
                        blockTimestampLast: None,
                    })
                    .collect::<Vec<Pair>>(),
            })
            .collect::<Vec<TokenMarket>>();
        Self {
            markets: pairs,
            flash_query_contract,
        }
    }

    pub async fn update_reserve(&mut self) {
        // let reserves = self.contract.get_reserves().call().await;
        // reserves
        let reserves = self
            .get_all_pair_addresses()
            .iter()
            .map(|pair| pair.address)
            .collect::<Vec<H160>>();

        let reserves = self
            .flash_query_contract
            .get_reserves_by_pairs(reserves)
            .call()
            .await
            .unwrap();

        for (reserve, pair) in std::iter::zip(&reserves, self.get_all_pair_addresses()) {
            pair.reserve0 = Some(reserve[0]);
            pair.reserve1 = Some(reserve[1]);
            pair.blockTimestampLast = Some(reserve[2]);
        }
        dbg!(self);

        // self.markets.
        // ()
    }

    fn get_all_pair_addresses(&mut self) -> Vec<&mut Pair> {
        self.markets
            .iter_mut()
            .flat_map(|token_market| &mut token_market.pairs)
            .collect::<Vec<&mut Pair>>()
    }
}

#[derive(Debug)]
pub struct TokenMarket<'a> {
    token: &'a H160,
    pairs: Vec<Pair>,
}

#[derive(Debug, Clone)]
pub struct Pair {
    address: H160,
    reserve0: Option<U256>,
    reserve1: Option<U256>,
    blockTimestampLast: Option<U256>,
}

#[cfg(test)]
mod test {
    #[test]
    fn is_flatten() {}
}
