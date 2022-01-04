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
                    .map(|address| Pair { address })
                    .collect::<Vec<Pair>>(),
            })
            .collect::<Vec<TokenMarket>>();
        Self {
            markets: pairs,
            flash_query_contract,
        }
    }

    pub async fn update_reserve(&self) {
        // let reserves = self.contract.get_reserves().call().await;
        // reserves
        let reserves = self
            .flatten()
            .into_iter()
            .map(|pair| pair.address)
            .collect::<Vec<H160>>();

        let reserves = self
            .flash_query_contract
            .get_reserves_by_pairs(reserves)
            .call()
            .await
            .unwrap();

        &self.markets.into_iter().flatten();

        dbg!(reserves);
        ()
    }

    fn flatten(&self) -> Vec<Pair> {
        self.markets
            .iter()
            .flat_map(|token_market| token_market.pairs.to_vec())
            .collect::<Vec<Pair>>()
    }
}

#[derive(Debug)]
pub struct TokenMarket<'a> {
    token: &'a H160,
    pairs: Vec<Pair>,
}

impl<'a> IntoIterator for TokenMarket<'a> {
    type Item = &'a Pair;
    type IntoIter = TokenMarketIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenMarketIterator {
            market: self,
            outer: Some(&self.pairs[0]),
            inner: 0,
        }
    }
}

struct TokenMarketIterator<'a> {
    market: TokenMarket<'a>,
    outer: Option<&'a Pair>,
    inner: usize,
}

impl<'a> Iterator for TokenMarketIterator<'a> {
    type Item = &'a Pair;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.market.pairs.get(self.inner);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    address: H160,
}

#[cfg(test)]
mod test {
    #[test]
    fn is_flatten() {}
}
