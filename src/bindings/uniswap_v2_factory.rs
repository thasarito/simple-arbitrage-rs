pub use uniswapv2factory_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv2factory_mod {
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
    #[doc = "UniswapV2Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV2FACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"constructor\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"_feeToSetter\",\"type\":\"address\"}]},{\"type\":\"function\",\"name\":\"allPairs\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allPairsLength\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createPair\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"feeTo\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"feeToSetter\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getPair\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setFeeTo\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"_feeTo\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setFeeToSetter\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"_feeToSetter\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"PairCreated\",\"inputs\":[{\"name\":\"token0\",\"type\":\"address\",\"indexed\":true},{\"name\":\"token1\",\"type\":\"address\",\"indexed\":true},{\"name\":\"pair\",\"type\":\"address\",\"indexed\":false},{\"name\":\"\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapV2Factory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV2Factory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV2Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV2Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV2Factory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                UNISWAPV2FACTORY_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `allPairs` (0x1e3dd18b) function"]
        pub fn all_pairs(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([30, 61, 209, 139], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allPairsLength` (0x574f2ba3) function"]
        pub fn all_pairs_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([87, 79, 43, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createPair` (0xc9c65396) function"]
        pub fn create_pair(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([201, 198, 83, 150], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeTo` (0x017e7e58) function"]
        pub fn fee_to(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([1, 126, 126, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeToSetter` (0x094b7415) function"]
        pub fn fee_to_setter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([9, 75, 116, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPair` (0xe6a43905) function"]
        pub fn get_pair(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([230, 164, 57, 5], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeTo` (0xf46901ed) function"]
        pub fn set_fee_to(
            &self,
            fee_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 105, 1, 237], fee_to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeToSetter` (0xa2e74af6) function"]
        pub fn set_fee_to_setter(
            &self,
            fee_to_setter: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 231, 74, 246], fee_to_setter)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PairCreated` event"]
        pub fn pair_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PairCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PairCreatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "PairCreated",
        abi = "PairCreated(address,address,address,uint256)"
    )]
    pub struct PairCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers::core::types::Address,
        pub pair: ethers::core::types::Address,
        pub p3: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allPairs`function with signature `allPairs(uint256)` and selector `[30, 61, 209, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allPairs", abi = "allPairs(uint256)")]
    pub struct AllPairsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `allPairsLength`function with signature `allPairsLength()` and selector `[87, 79, 43, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allPairsLength", abi = "allPairsLength()")]
    pub struct AllPairsLengthCall;
    #[doc = "Container type for all input parameters for the `createPair`function with signature `createPair(address,address)` and selector `[201, 198, 83, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `feeTo`function with signature `feeTo()` and selector `[1, 126, 126, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "feeTo", abi = "feeTo()")]
    pub struct FeeToCall;
    #[doc = "Container type for all input parameters for the `feeToSetter`function with signature `feeToSetter()` and selector `[9, 75, 116, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "feeToSetter", abi = "feeToSetter()")]
    pub struct FeeToSetterCall;
    #[doc = "Container type for all input parameters for the `getPair`function with signature `getPair(address,address)` and selector `[230, 164, 57, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPair", abi = "getPair(address,address)")]
    pub struct GetPairCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `setFeeTo`function with signature `setFeeTo(address)` and selector `[244, 105, 1, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFeeTo", abi = "setFeeTo(address)")]
    pub struct SetFeeToCall {
        pub fee_to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFeeToSetter`function with signature `setFeeToSetter(address)` and selector `[162, 231, 74, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFeeToSetter", abi = "setFeeToSetter(address)")]
    pub struct SetFeeToSetterCall {
        pub fee_to_setter: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV2FactoryCalls {
        AllPairs(AllPairsCall),
        AllPairsLength(AllPairsLengthCall),
        CreatePair(CreatePairCall),
        FeeTo(FeeToCall),
        FeeToSetter(FeeToSetterCall),
        GetPair(GetPairCall),
        SetFeeTo(SetFeeToCall),
        SetFeeToSetter(SetFeeToSetterCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV2FactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllPairsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::AllPairs(decoded));
            }
            if let Ok(decoded) =
                <AllPairsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::AllPairsLength(decoded));
            }
            if let Ok(decoded) =
                <CreatePairCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::CreatePair(decoded));
            }
            if let Ok(decoded) = <FeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::FeeTo(decoded));
            }
            if let Ok(decoded) =
                <FeeToSetterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::FeeToSetter(decoded));
            }
            if let Ok(decoded) =
                <GetPairCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::GetPair(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::SetFeeTo(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToSetterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2FactoryCalls::SetFeeToSetter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV2FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV2FactoryCalls::AllPairs(element) => element.encode(),
                UniswapV2FactoryCalls::AllPairsLength(element) => element.encode(),
                UniswapV2FactoryCalls::CreatePair(element) => element.encode(),
                UniswapV2FactoryCalls::FeeTo(element) => element.encode(),
                UniswapV2FactoryCalls::FeeToSetter(element) => element.encode(),
                UniswapV2FactoryCalls::GetPair(element) => element.encode(),
                UniswapV2FactoryCalls::SetFeeTo(element) => element.encode(),
                UniswapV2FactoryCalls::SetFeeToSetter(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV2FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV2FactoryCalls::AllPairs(element) => element.fmt(f),
                UniswapV2FactoryCalls::AllPairsLength(element) => element.fmt(f),
                UniswapV2FactoryCalls::CreatePair(element) => element.fmt(f),
                UniswapV2FactoryCalls::FeeTo(element) => element.fmt(f),
                UniswapV2FactoryCalls::FeeToSetter(element) => element.fmt(f),
                UniswapV2FactoryCalls::GetPair(element) => element.fmt(f),
                UniswapV2FactoryCalls::SetFeeTo(element) => element.fmt(f),
                UniswapV2FactoryCalls::SetFeeToSetter(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllPairsCall> for UniswapV2FactoryCalls {
        fn from(var: AllPairsCall) -> Self {
            UniswapV2FactoryCalls::AllPairs(var)
        }
    }
    impl ::std::convert::From<AllPairsLengthCall> for UniswapV2FactoryCalls {
        fn from(var: AllPairsLengthCall) -> Self {
            UniswapV2FactoryCalls::AllPairsLength(var)
        }
    }
    impl ::std::convert::From<CreatePairCall> for UniswapV2FactoryCalls {
        fn from(var: CreatePairCall) -> Self {
            UniswapV2FactoryCalls::CreatePair(var)
        }
    }
    impl ::std::convert::From<FeeToCall> for UniswapV2FactoryCalls {
        fn from(var: FeeToCall) -> Self {
            UniswapV2FactoryCalls::FeeTo(var)
        }
    }
    impl ::std::convert::From<FeeToSetterCall> for UniswapV2FactoryCalls {
        fn from(var: FeeToSetterCall) -> Self {
            UniswapV2FactoryCalls::FeeToSetter(var)
        }
    }
    impl ::std::convert::From<GetPairCall> for UniswapV2FactoryCalls {
        fn from(var: GetPairCall) -> Self {
            UniswapV2FactoryCalls::GetPair(var)
        }
    }
    impl ::std::convert::From<SetFeeToCall> for UniswapV2FactoryCalls {
        fn from(var: SetFeeToCall) -> Self {
            UniswapV2FactoryCalls::SetFeeTo(var)
        }
    }
    impl ::std::convert::From<SetFeeToSetterCall> for UniswapV2FactoryCalls {
        fn from(var: SetFeeToSetterCall) -> Self {
            UniswapV2FactoryCalls::SetFeeToSetter(var)
        }
    }
}
