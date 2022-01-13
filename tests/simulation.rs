mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {

    use super::*;
    use ethers::{
        prelude::{artifacts::CompactContract, U256},
        utils::Ganache,
    };

    #[tokio::test]
    async fn test_arbitrage() {
        let ganache = Ganache::new().arg("--allowUnlimitedContractSize").spawn();

        let provider = connect(&ganache, 0);
        // let searcher = connect(&ganache, 1);
        // Start deploy token
        let compact_weth: CompactContract =
            serde_json::from_str(include_str!("../out/WETH9.sol/WETH9.json")).unwrap();
        let weth = deploy(compact_weth, provider.clone()).await;
        let weth = weth.deploy(()).unwrap().legacy().send().await.unwrap();

        let compact_token: CompactContract =
            serde_json::from_str(include_str!("../out/TToken.sol/TToken.json")).unwrap();
        let token = deploy(compact_token, provider.clone()).await;
        let token = token.deploy(()).unwrap().legacy().send().await.unwrap();
        // Finish deploy token

        let token_balance_1 = U256::from_dec_str("26736768576059").unwrap();
        let eth_balance_1 = U256::from_dec_str("9561078446416170138").unwrap();

        create_pool(
            token.address(),
            weth.address(),
            token_balance_1,
            eth_balance_1,
            provider.clone(),
        )
        .await;

        let token_balance_2 = U256::from_dec_str("27402034049012").unwrap();
        let eth_balance_2 = U256::from_dec_str("8581483325062417688").unwrap();
        create_pool(
            token.address(),
            weth.address(),
            token_balance_2,
            eth_balance_2,
            provider.clone(),
        )
        .await;
    }
}
