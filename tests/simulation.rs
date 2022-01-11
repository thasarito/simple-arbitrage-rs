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

        let compactFactory: CompactContract = serde_json::from_str(include_str!(
            "../out/UniswapV2Factory.sol/UniswapV2Factory.json"
        ))
        .unwrap();
        let (abi, bytes, _) = compactFactory.into_parts_or_default();
        let factoryFactory = ContractFactory::new(abi, bytes, client1.clone());

        let factoryAddrs = factoryFactory
            .deploy(H160::zero())
            .unwrap()
            .legacy()
            .send()
            .await
            .unwrap()
            .address();

        dbg!(factoryAddrs);
    }
}
