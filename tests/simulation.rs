mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {
    use std::sync::Arc;

    use super::*;
    use ethers::{prelude::H160, utils::Ganache};
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

        let factory = compile_contract("UniswapV2Factory", "UniswapV2Factory.sol");
        let router = compile_contract("UniswapV2Router", "UniswapV2Router.sol");
        // let factoryAddrs = factory
        //     .deploy(H160::zero())
        //     .unwrap()
        //     .legacy()
        //     .send()
        //     .await
        //     .unwrap()
        //     .address();

        // dbg!(factoryAddrs);
    }
}
