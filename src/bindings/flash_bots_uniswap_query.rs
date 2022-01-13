pub use flashbotsuniswapquery_mod::*;
#[allow(clippy::too_many_arguments)]
mod flashbotsuniswapquery_mod {
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
    #[doc = "FlashBotsUniswapQuery was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FLASHBOTSUNISWAPQUERY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"function\",\"name\":\"getPairsByIndexRange\",\"inputs\":[{\"internalType\":\"contract IUniswapV2Factory\",\"name\":\"_uniswapFactory\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_start\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_stop\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"address[3][]\",\"name\":\"\",\"type\":\"address[3][]\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getReservesByPairs\",\"inputs\":[{\"internalType\":\"contract IUniswapV2Pair[]\",\"name\":\"_pairs\",\"type\":\"address[]\"}],\"outputs\":[{\"internalType\":\"uint256[3][]\",\"name\":\"\",\"type\":\"uint256[3][]\"}],\"constant\":false,\"stateMutability\":\"view\"}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct FlashBotsUniswapQuery<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for FlashBotsUniswapQuery<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for FlashBotsUniswapQuery<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FlashBotsUniswapQuery))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> FlashBotsUniswapQuery<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                FLASHBOTSUNISWAPQUERY_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `getPairsByIndexRange` (0xab2217e4) function"]
        pub fn get_pairs_by_index_range(
            &self,
            uniswap_factory: ethers::core::types::Address,
            start: ethers::core::types::U256,
            stop: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<[ethers::core::types::Address; 3usize]>,
        > {
            self.0
                .method_hash([171, 34, 23, 228], (uniswap_factory, start, stop))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReservesByPairs` (0x4dbf0f39) function"]
        pub fn get_reserves_by_pairs(
            &self,
            pairs: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<[ethers::core::types::U256; 3usize]>,
        > {
            self.0
                .method_hash([77, 191, 15, 57], pairs)
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `getPairsByIndexRange`function with signature `getPairsByIndexRange(address,uint256,uint256)` and selector `[171, 34, 23, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getPairsByIndexRange",
        abi = "getPairsByIndexRange(address,uint256,uint256)"
    )]
    pub struct GetPairsByIndexRangeCall {
        pub uniswap_factory: ethers::core::types::Address,
        pub start: ethers::core::types::U256,
        pub stop: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getReservesByPairs`function with signature `getReservesByPairs(address[])` and selector `[77, 191, 15, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReservesByPairs", abi = "getReservesByPairs(address[])")]
    pub struct GetReservesByPairsCall {
        pub pairs: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FlashBotsUniswapQueryCalls {
        GetPairsByIndexRange(GetPairsByIndexRangeCall),
        GetReservesByPairs(GetReservesByPairsCall),
    }
    impl ethers::core::abi::AbiDecode for FlashBotsUniswapQueryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetPairsByIndexRangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashBotsUniswapQueryCalls::GetPairsByIndexRange(decoded));
            }
            if let Ok(decoded) =
                <GetReservesByPairsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashBotsUniswapQueryCalls::GetReservesByPairs(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FlashBotsUniswapQueryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FlashBotsUniswapQueryCalls::GetPairsByIndexRange(element) => element.encode(),
                FlashBotsUniswapQueryCalls::GetReservesByPairs(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FlashBotsUniswapQueryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FlashBotsUniswapQueryCalls::GetPairsByIndexRange(element) => element.fmt(f),
                FlashBotsUniswapQueryCalls::GetReservesByPairs(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetPairsByIndexRangeCall> for FlashBotsUniswapQueryCalls {
        fn from(var: GetPairsByIndexRangeCall) -> Self {
            FlashBotsUniswapQueryCalls::GetPairsByIndexRange(var)
        }
    }
    impl ::std::convert::From<GetReservesByPairsCall> for FlashBotsUniswapQueryCalls {
        fn from(var: GetReservesByPairsCall) -> Self {
            FlashBotsUniswapQueryCalls::GetReservesByPairs(var)
        }
    }
}
