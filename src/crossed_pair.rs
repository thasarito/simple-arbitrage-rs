use crate::{addresses::WETH_ADDRESS, bindings::flash_bots_uniswap_query::FlashBotsUniswapQuery};
use ethers::{abi::ethereum_types::U512, prelude::*};

#[derive(Debug)]
pub struct CrossedPairManager<'a, M>
where
    M: Middleware,
{
    flash_query_contract: &'a FlashBotsUniswapQuery<M>,
    markets: Vec<TokenMarket<'a>>,
}

impl<'a, M> CrossedPairManager<'a, M>
where
    M: Middleware,
{
    pub fn new(
        grouped_pairs: &'a [(H160, Vec<[H160; 3]>)],
        flash_query_contract: &'a FlashBotsUniswapQuery<M>,
    ) -> Self {
        let pairs = grouped_pairs
            .iter()
            .map(|(token, pairs)| TokenMarket {
                token,
                pairs: pairs
                    .iter()
                    .copied()
                    .map(|[token0, token1, address]| Pair {
                        address,
                        token0,
                        token1,
                        reserve: None,
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

        for (new_reserve, pair) in std::iter::zip(&reserves, self.get_all_pair_addresses()) {
            let weth_address = &WETH_ADDRESS.parse::<Address>().unwrap();
            let (reserve0, reserve1) = if &pair.token0 == weth_address {
                (new_reserve[1], new_reserve[0])
            } else {
                (new_reserve[0], new_reserve[1])
            };
            let updated_reserve = Reserve {
                reserve0,
                reserve1,
                // block_timestamp_last: new_reserve[2],
            };

            pair.reserve = Some(updated_reserve);
        }
    }

    fn get_all_pair_addresses(&mut self) -> Vec<&mut Pair> {
        self.markets
            .iter_mut()
            .flat_map(|token_market| &mut token_market.pairs)
            .collect::<Vec<&mut Pair>>()
    }

    pub fn find_arbitrage_opportunities(&mut self) {
        for market in &mut self.markets {
            market.find_arbitrage_opportunity();
        }
    }
}

#[derive(Debug)]
pub struct TokenMarket<'a> {
    token: &'a H160,
    pairs: Vec<Pair>,
}

impl<'a> TokenMarket<'a> {
    pub fn find_arbitrage_opportunity(&self) {
        for pair_a in &self.pairs {
            for pair_b in &self.pairs {
                if let Some((x, _alt_amount, profit)) = profit(
                    pair_a.reserve.as_ref().unwrap(),
                    pair_b.reserve.as_ref().unwrap(),
                ) {
                    if profit.gt(&U512::from(10u128.pow(15))) {
                        dbg!(self.token, x);
                        dbg!(pair_a.address, pair_b.address);
                        dbg!(
                            pair_a.reserve.as_ref().unwrap(),
                            pair_b.reserve.as_ref().unwrap()
                        );
                        dbg!(profit);
                        let profit_usd = (profit.as_u128() as f64) / 10f64.powf(18f64) * 4000f64;
                        dbg!(profit_usd);
                        println!("------------------------------------------------------------------------------------------");
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Pair {
    address: H160,
    token0: H160,
    token1: H160,
    reserve: Option<Reserve>,
}

#[derive(Debug)]
pub struct Reserve {
    reserve0: U256,
    reserve1: U256,
}

impl Reserve {
    pub fn new(reserve0: U256, reserve1: U256) -> Self {
        Self { reserve0, reserve1 }
    }
}

pub fn profit(pair_a: &Reserve, pair_b: &Reserve) -> Option<(U512, U512, U512)> {
    // Uniswap return U112
    let q = U512::from(pair_a.reserve0 * pair_b.reserve1);
    let r = U512::from(pair_b.reserve0 * pair_a.reserve1);
    let s = U512::from(pair_a.reserve0 + pair_b.reserve0);
    if r > q {
        return None;
    }

    let r2 = r.checked_pow(U512::from(2i32)).expect("power overflow");
    let x_opt = (r2 + ((q * r - r2) / s)).integer_sqrt() - r;
    if x_opt == U512::from(0u128) {
        return None;
    }
    let alt_amount = U512::from(pair_a.reserve0) * x_opt / (U512::from(pair_a.reserve1) + x_opt);
    let p = (q * x_opt) / (r + s * x_opt) - x_opt;

    Some((x_opt, alt_amount, p))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn found_arbitrage() {
        let uniswap_reserve = Reserve {
            reserve0: U256::from_dec_str("32417025234557782261").unwrap(),
            reserve1: U256::from_dec_str("322112290808754715658").unwrap(),
        };

        let sushi_reserve = Reserve {
            reserve0: U256::from_dec_str("385013293957127603432").unwrap(),
            reserve1: U256::from_dec_str("4864221907791931816675").unwrap(),
        };

        let (x, alt, profit) = profit(&uniswap_reserve, &sushi_reserve).unwrap();
        dbg!(x, alt, profit);
    }
}
