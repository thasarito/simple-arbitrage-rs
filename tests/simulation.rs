mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {
    use std::ops::Mul;

    use super::*;
    use ethers::{
        prelude::{artifacts::CompactContract, Middleware, H160, U256},
        utils::Ganache,
    };
    use forge_test::bindings::{
        t_token::TToken, uniswap_v2_router_02::UniswapV2Router02, weth9::WETH9,
    };

    #[tokio::test]
    async fn test_arbitrage() {
        let ganache = Ganache::new().arg("--allowUnlimitedContractSize").spawn();

        let addrs = ganache.addresses().to_vec();
        let deployer = addrs[0];

        let provider = connect(&ganache, 0);
        // let searcher = connect(&ganache, 1);

        let compact_factory: CompactContract =
            serde_json::from_str(include_str!("../out/UniswapV2Factory.json")).unwrap();
        let factory = deploy(compact_factory, provider.clone()).await;
        let factory = factory
            .deploy(H160::zero())
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let compact_weth: CompactContract =
            serde_json::from_str(include_str!("../out/WETH9.sol/WETH9.json")).unwrap();
        let weth = deploy(compact_weth, provider.clone()).await;
        let weth = weth.deploy(()).unwrap().legacy().send().await.unwrap();

        let compact_token: CompactContract =
            serde_json::from_str(include_str!("../out/TToken.sol/TToken.json")).unwrap();
        let token = deploy(compact_token, provider.clone()).await;
        let token = token.deploy(()).unwrap().legacy().send().await.unwrap();

        let compact_router: CompactContract =
            serde_json::from_str(include_str!("../out/UniswapV2Router02.json")).unwrap();

        let router = deploy(compact_router, provider.clone()).await;
        let router = router
            .deploy((factory.address(), weth.address()))
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let block = provider.get_block_number().await.unwrap();
        let time = provider.get_block(block).await.unwrap().unwrap().timestamp;

        let searcher_token = TToken::new(token.address(), provider.clone());
        // let token_balance = searcher_token.balance_of(deployer).call().await.unwrap();
        // // dbg!(token_balance);
        let token_balance_1 = U256::from_dec_str("26736768576059").unwrap();
        let eth_balance_1 = U256::from_dec_str("9561078446416170138").unwrap();

        searcher_token
            .approve(router.address(), U256::MAX)
            .legacy()
            .send()
            .await
            .unwrap();

        let allowance = searcher_token
            .allowance(deployer, router.address())
            .call()
            .await
            .unwrap();
        dbg!(allowance);

        let weth_provider = WETH9::new(weth.address(), provider.clone());
        weth_provider
            .deposit()
            .value(eth_balance_1)
            .legacy()
            .send()
            .await
            .unwrap();
        let deployer_weth = weth_provider.balance_of(deployer).call().await.unwrap();
        dbg!(deployer_weth);
        weth_provider
            .approve(router.address(), U256::MAX)
            .legacy()
            .send()
            .await
            .unwrap();

        let searcher_router = UniswapV2Router02::new(router.address(), provider.clone());
        searcher_router
            .add_liquidity_eth(
                token.address(),
                token_balance_1,
                U256::zero(),
                U256::zero(),
                deployer,
                time.mul(2u32),
            )
            .legacy()
            .value(eth_balance_1)
            .send()
            .await
            .unwrap();
    }
}
