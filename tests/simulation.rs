mod common;
pub use common::*;

#[cfg(test)]
mod simulation_test {
    use std::sync::Arc;

    use super::*;
    use ethers::utils::Ganache;
    use forge_test::bindings::uniswap_v2_router::UniswapV2Router;

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
        let addr1 = addrs[0];
        let addr2 = addrs[1];

        let client1 = connect(&ganache, 0);
        let client2 = connect(&ganache, 1);
        let router = UniswapV2Router::new(addr1, Arc::clone(&client1));
        dbg!(router);
    }
}
