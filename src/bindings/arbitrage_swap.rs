pub use arbitrageswap_mod::*;
#[allow(clippy::too_many_arguments)]
mod arbitrageswap_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ArbitrageSwap was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ARBITRAGESWAP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"constructor\",\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_weth\",\"type\":\"address\"}]},{\"type\":\"function\",\"name\":\"swap\",\"inputs\":[{\"internalType\":\"contract IUniswapV2Pair\",\"name\":\"pool_a\",\"type\":\"address\"},{\"internalType\":\"contract IUniswapV2Pair\",\"name\":\"pool_b\",\"type\":\"address\"},{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"intermediate_amount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"profit\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"payable\"},{\"type\":\"receive\"}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ArbitrageSwap<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ArbitrageSwap<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ArbitrageSwap<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ArbitrageSwap))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ArbitrageSwap<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), ARBITRAGESWAP_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `swap` (0xe343fe12) function"]
        pub fn swap(
            &self,
            pool_a: ethers::core::types::Address,
            pool_b: ethers::core::types::Address,
            token: ethers::core::types::Address,
            intermediate_amount: ethers::core::types::U256,
            profit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 67, 254, 18],
                    (pool_a, pool_b, token, intermediate_amount, profit),
                )
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `swap`function with signature `swap(address,address,address,uint256,uint256)` and selector `[227, 67, 254, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "swap", abi = "swap(address,address,address,uint256,uint256)")]
    pub struct SwapCall {
        pub pool_a: ethers::core::types::Address,
        pub pool_b: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub intermediate_amount: ethers::core::types::U256,
        pub profit: ethers::core::types::U256,
    }
}
