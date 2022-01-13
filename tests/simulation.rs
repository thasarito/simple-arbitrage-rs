mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {
    use std::{ops::Mul, sync::Arc};

    use super::*;
    use ethers::{
        prelude::{artifacts::CompactContract, Artifact, ContractFactory, Middleware, H160, U256},
        utils::Ganache,
    };
    use forge_test::bindings::{
        erc20::ERC20, uniswap_v2_factory::UniswapV2Factory, uniswap_v2_router::UniswapV2Router,
    };

    #[tokio::test]
    async fn test_arbitrage() {
        let ganache = Ganache::new().arg("--allowUnlimitedContractSize").spawn();

        let addrs = ganache.addresses().to_vec();
        let deployer = addrs[0];
        let addr2 = addrs[1];

        let provider = connect(&ganache, 0);
        let searcher = connect(&ganache, 1);

        let compact_factory: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Factory.sol/UniswapV2Factory.json"
        ))
        .unwrap();
        let factory = deploy(compact_factory, provider.clone()).await;
        let factory = factory
            .deploy(H160::zero())
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let compact_pair: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Factory.sol/UniswapV2Pair.json"
        ))
        .unwrap();
        let pair = deploy(compact_pair, provider.clone()).await;

        let compact_weth: CompactContract =
            serde_json::from_str(include_str!("../out/WETH9.sol/WETH9.json")).unwrap();
        let weth = deploy(compact_weth, provider.clone()).await;
        let weth = weth.deploy(()).unwrap().legacy().send().await.unwrap();

        let compact_token: CompactContract =
            serde_json::from_str(include_str!("../out/ERC20.sol/ERC20.json")).unwrap();
        let token = deploy(compact_token, provider.clone()).await;
        let token = token
            .deploy(("Token1".to_string(), "TK1".to_string()))
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let compact_router: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Router.sol/UniswapV2Router02.json"
        ))
        .unwrap();

        let balance = provider.get_balance(deployer, None).await.unwrap();
        dbg!(balance);
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

        let searcher_token = ERC20::new(token.address(), searcher.clone());
        let token_balance_1 = U256::from_dec_str("26736768576059172").unwrap();
        let eth_balance_1 = U256::from_dec_str("9561078446416170138885").unwrap();

        searcher_token
            .approve(router.address(), token_balance_1)
            .call()
            .await
            .unwrap();

        let searcher_router = UniswapV2Router::new(router.address(), searcher.clone());

        searcher_router
            .add_liquidity_eth(
                token.address(),
                token_balance_1,
                U256::zero(),
                U256::zero(),
                deployer,
                time.mul(2u32),
            )
            .value(eth_balance_1)
            .call()
            .await
            .unwrap();
    }
}
