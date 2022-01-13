use std::sync::Arc;
use std::time::Duration;

// use ethers::prelude::artifacts::{CompactContract, Contract};
use ethers::prelude::artifacts::CompactContract;
use ethers::prelude::*;
use ethers::utils::GanacheInstance;

use ethers::providers::Http;
use forge_test::bindings::t_token::TToken;
use forge_test::bindings::uniswap_v2_router_02::UniswapV2Router02;

// connects the private key to http://localhost:8545
pub fn connect(ganache: &GanacheInstance, idx: usize) -> Arc<Provider<Http>> {
    let sender = ganache.addresses()[idx];
    let provider = Provider::<Http>::try_from(ganache.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64))
        .with_sender(sender);
    Arc::new(provider)
}

pub async fn deploy<T>(
    compact_contract: CompactContract,
    client: Arc<Provider<T>>,
) -> ContractFactory<Provider<T>>
where
    T: JsonRpcClient,
{
    let (abi, bytes, _) = compact_contract.into_parts_or_default();
    let factory = ContractFactory::new(abi, bytes, client);

    factory
}

pub async fn create_pool<T>(
    token: H160,
    weth: H160,
    amount_token: U256,
    amount_eth: U256,
    client: Arc<Provider<T>>,
) where
    T: JsonRpcClient,
{
    let deployer = client.default_sender().unwrap();
    let compact_factory: CompactContract =
        serde_json::from_str(include_str!("../out/UniswapV2Factory.json")).unwrap();
    let factory = deploy(compact_factory, client.clone()).await;
    let factory = factory
        .deploy(H160::zero())
        .unwrap()
        .legacy()
        .send()
        .await
        .unwrap();

    let compact_router: CompactContract =
        serde_json::from_str(include_str!("../out/UniswapV2Router02.json")).unwrap();

    let router = deploy(compact_router, client.clone()).await;
    let router = router
        .deploy((factory.address(), weth))
        .unwrap()
        .legacy()
        .send()
        .await
        .unwrap();

    let block = client.get_block_number().await.unwrap();
    let time = client.get_block(block).await.unwrap().unwrap().timestamp;

    let searcher_token = TToken::new(token, client.clone());

    searcher_token
        .approve(router.address(), U256::MAX)
        .legacy()
        .send()
        .await
        .unwrap();

    let searcher_router = UniswapV2Router02::new(router.address(), client.clone());
    searcher_router
        .add_liquidity_eth(
            token,
            amount_token,
            U256::zero(),
            U256::zero(),
            deployer,
            time * (2u32),
        )
        .legacy()
        .value(amount_eth)
        .send()
        .await
        .unwrap();
}
