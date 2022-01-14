mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {

    use super::*;
    use ethers::{
        prelude::{artifacts::CompactContract, U256},
        utils::Ganache,
    };
    use forge_test::crossed_pair::{profit, Reserve};

    #[tokio::test]
    async fn test_arbitrage() {
        let ganache = Ganache::new()
            .arg("--allowUnlimitedContractSize")
            .arg("-e")
            .arg("1000000")
            .spawn();

        let provider = connect(&ganache, 0);

        let compact_weth: CompactContract =
            serde_json::from_str(include_str!("../out/WETH9.sol/WETH9.json")).unwrap();
        let weth = deploy(compact_weth, provider.clone()).await;
        let weth = weth.deploy(()).unwrap().legacy().send().await.unwrap();

        let compact_token: CompactContract =
            serde_json::from_str(include_str!("../out/TToken.sol/TToken.json")).unwrap();
        let token = deploy(compact_token, provider.clone()).await;
        let token = token.deploy(()).unwrap().legacy().send().await.unwrap();
        // Finish deploy token

        let token_balance_1 = U256::from_dec_str("32417025234557782261").unwrap();
        let eth_balance_1 = U256::from_dec_str("322112290808754715658").unwrap();

        let pool_a = create_pool(
            token.address(),
            weth.address(),
            token_balance_1,
            eth_balance_1,
            provider.clone(),
        )
        .await;

        let token_balance_2 = U256::from_dec_str("385013293957127603432").unwrap();
        let eth_balance_2 = U256::from_dec_str("4864221907791931816675").unwrap();
        let pool_b = create_pool(
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
        dbg!(x, intm, prof);
    }
}
