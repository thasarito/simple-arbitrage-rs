mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {

    use super::*;
    use ethers::{
        prelude::{artifacts::CompactContract, Middleware, U256},
        utils::Ganache,
    };
    use forge_test::{
        bindings::{arbitrage_swap::ArbitrageSwap, weth9::WETH9},
        crossed_pair::{profit, Reserve},
    };

    #[tokio::test]
    async fn test_arbitrage() {
        let ganache = Ganache::new()
            .arg("--allowUnlimitedContractSize")
            .arg("-e")
            .arg("1000000")
            .spawn();

        let provider = connect(&ganache, 0);
        let searcher = connect(&ganache, 1);

        let compact_weth: CompactContract =
            serde_json::from_str(include_str!("../out/WETH9.sol/WETH9.json")).unwrap();
        let weth = deploy(compact_weth, provider.clone()).await;
        let weth = weth.deploy(()).unwrap().legacy().send().await.unwrap();

        let compact_arb: CompactContract =
            serde_json::from_str(include_str!("../out/ArbitrageSwap.sol/ArbitrageSwap.json"))
                .unwrap();
        let arb = deploy(compact_arb, provider.clone()).await;
        let arb = arb
            .deploy(weth.address())
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let compact_token: CompactContract =
            serde_json::from_str(include_str!("../out/TToken.sol/TToken.json")).unwrap();
        let token = deploy(compact_token, provider.clone()).await;
        let token = token.deploy(()).unwrap().legacy().send().await.unwrap();
        // Finish deploy token

        let token_balance_1 = U256::from_dec_str("32417025234557782261").unwrap();
        let eth_balance_1 = U256::from_dec_str("322112290808754715658").unwrap();

        let (_factory_a, _router_a, pool_a) = create_pool(
            token.address(),
            weth.address(),
            token_balance_1,
            eth_balance_1,
            provider.clone(),
        )
        .await;

        let token_balance_2 = U256::from_dec_str("385013293957127603432").unwrap();
        let eth_balance_2 = U256::from_dec_str("4864221907791931816675").unwrap();
        let (_factory_b, _router_b, pool_b) = create_pool(
            token.address(),
            weth.address(),
            token_balance_2,
            eth_balance_2,
            provider.clone(),
        )
        .await;

        let pair_a = Reserve::new(token_balance_1, eth_balance_1);
        let pair_b = Reserve::new(token_balance_2, eth_balance_2);
        let (x, intm, prof) = profit(&pair_a, &pair_b).unwrap();
        let intm = intm * 997u32 / 1000u32;
        let prof = prof * 997u32 / 1000u32;
        dbg!(x, intm, prof, pair_a, pair_b);

        // let router_a = UniswapV2Router02::new(router_a, searcher.clone());
        // let router_b = UniswapV2Router02::new(router_b, searcher.clone());
        // let block = searcher.get_block_number().await.unwrap();

        // Insufficient liquidity vs revert
        let initial_balance = searcher
            .get_balance(searcher.default_sender().unwrap(), None)
            .await
            .unwrap();
        dbg!(initial_balance);
        let arb = ArbitrageSwap::new(arb.address(), searcher.clone());
        arb.swap(
            pool_a,
            pool_b,
            token.address(),
            U256::try_from(intm).unwrap(),
            U256::try_from(prof).unwrap(),
        )
        .value(U256::try_from(x).unwrap())
        .legacy()
        .send()
        .await
        .unwrap();

        let weth_call = WETH9::new(weth.address(), searcher.clone());
        let after_balance = weth_call
            .balance_of(searcher.default_sender().unwrap())
            .call()
            .await
            .unwrap();
        dbg!(after_balance);
    }
}
