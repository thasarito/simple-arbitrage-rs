mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {
    use std::sync::Arc;

    use super::*;
    use ethers::{
        prelude::{artifacts::CompactContract, Artifact, ContractFactory, H160},
        utils::Ganache,
    };
    use forge_test::bindings::{
        uniswap_v2_factory::UniswapV2Factory, uniswap_v2_router::UniswapV2Router,
    };

    #[tokio::test]
    async fn test_arbitrage() {
        // uniswap_v2_factory;
        // uniswapv2factory::

        // let (client, _mock) = Provider::mocked();
        // let contract = StakedOHM::new(Address::default(), Arc::new(client));

        // let _ = contract.index();
        // let _ = contract.INDEX();
        let ganache = Ganache::new().spawn();

        let addrs = ganache.addresses().to_vec();
        let deployer = addrs[0];
        let addr2 = addrs[1];

        let client1 = connect(&ganache, 0);

        let compact_factory: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Factory.sol/UniswapV2Factory.json"
        ))
        .unwrap();
        let factory = deploy(compact_factory, client1.clone()).await;
        let factory = factory
            .deploy(H160::zero())
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let compact_router: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Router.sol/UniswapV2Router02.json"
        ))
        .unwrap();
        let router = deploy(compact_router, client1.clone()).await;
        let router = router
            .deploy(factory.address())
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap();

        let compact_pair: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Factory.sol/UniswapV2Pair.json"
        ))
        .unwrap();
        let pair = deploy(compact_pair, client1.clone()).await;
    }
}
