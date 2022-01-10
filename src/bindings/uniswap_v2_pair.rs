pub use uniswapv2pair_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv2pair_mod {
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
    #[doc = "UniswapV2Pair was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV2PAIR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"constructor\",\"inputs\":[]},{\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"MINIMUM_LIQUIDITY\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"PERMIT_TYPEHASH\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allowance\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"approve\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"balanceOf\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"burn\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"decimals\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"factory\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getReserves\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint112\",\"name\":\"_reserve0\",\"type\":\"uint112\"},{\"internalType\":\"uint112\",\"name\":\"_reserve1\",\"type\":\"uint112\"},{\"internalType\":\"uint32\",\"name\":\"_blockTimestampLast\",\"type\":\"uint32\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_token1\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"kLast\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"mint\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"name\",\"inputs\":[],\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nonces\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"permit\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"price0CumulativeLast\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"price1CumulativeLast\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"skim\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"swap\",\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0Out\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Out\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"symbol\",\"inputs\":[],\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"sync\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"token0\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"token1\",\"inputs\":[],\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"totalSupply\",\"inputs\":[],\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transfer\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferFrom\",\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Approval\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\",\"indexed\":true},{\"name\":\"spender\",\"type\":\"address\",\"indexed\":true},{\"name\":\"value\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Burn\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"indexed\":true},{\"name\":\"amount0\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"amount1\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"to\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Mint\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"indexed\":true},{\"name\":\"amount0\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"amount1\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Swap\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"indexed\":true},{\"name\":\"amount0In\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"amount1In\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"amount0Out\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"amount1Out\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"to\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Sync\",\"inputs\":[{\"name\":\"reserve0\",\"type\":\"uint112\",\"indexed\":false},{\"name\":\"reserve1\",\"type\":\"uint112\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Transfer\",\"inputs\":[{\"name\":\"from\",\"type\":\"address\",\"indexed\":true},{\"name\":\"to\",\"type\":\"address\",\"indexed\":true},{\"name\":\"value\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapV2Pair<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV2Pair<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV2Pair<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV2Pair))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV2Pair<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), UNISWAPV2PAIR_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINIMUM_LIQUIDITY` (0xba9a7a56) function"]
        pub fn minimum_liquidity(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 154, 122, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x89afcb44) function"]
        pub fn burn(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([137, 175, 203, 68], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserves` (0x0902f1ac) function"]
        pub fn get_reserves(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128, u32)> {
            self.0
                .method_hash([9, 2, 241, 172], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x485cc955) function"]
        pub fn initialize(
            &self,
            token_0: ethers::core::types::Address,
            token_1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (token_0, token_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `kLast` (0x7464fc3d) function"]
        pub fn k_last(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([116, 100, 252, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x6a627842) function"]
        pub fn mint(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([106, 98, 120, 66], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `price0CumulativeLast` (0x5909c0d5) function"]
        pub fn price_0_cumulative_last(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([89, 9, 192, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `price1CumulativeLast` (0x5a3d5493) function"]
        pub fn price_1_cumulative_last(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 61, 84, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `skim` (0xbc25cf77) function"]
        pub fn skim(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 37, 207, 119], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x022c0d9f) function"]
        pub fn swap(
            &self,
            amount_0_out: ethers::core::types::U256,
            amount_1_out: ethers::core::types::U256,
            to: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 44, 13, 159], (amount_0_out, amount_1_out, to, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sync` (0xfff6cae9) function"]
        pub fn sync(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 246, 202, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Sync` event"]
        pub fn sync_filter(&self) -> ethers::contract::builders::Event<M, SyncFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UniswapV2PairEvents> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,uint256,uint256,address)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
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
        name = "Swap",
        abi = "Swap(address,uint256,uint256,uint256,uint256,address)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub amount_0_in: ethers::core::types::U256,
        pub amount_1_in: ethers::core::types::U256,
        pub amount_0_out: ethers::core::types::U256,
        pub amount_1_out: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
    #[ethevent(name = "Sync", abi = "Sync(uint112,uint112)")]
    pub struct SyncFilter {
        pub reserve_0: u128,
        pub reserve_1: u128,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV2PairEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        SyncFilter(SyncFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for UniswapV2PairEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SyncFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::SyncFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(UniswapV2PairEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UniswapV2PairEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV2PairEvents::ApprovalFilter(element) => element.fmt(f),
                UniswapV2PairEvents::BurnFilter(element) => element.fmt(f),
                UniswapV2PairEvents::MintFilter(element) => element.fmt(f),
                UniswapV2PairEvents::SwapFilter(element) => element.fmt(f),
                UniswapV2PairEvents::SyncFilter(element) => element.fmt(f),
                UniswapV2PairEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR`function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `MINIMUM_LIQUIDITY`function with signature `MINIMUM_LIQUIDITY()` and selector `[186, 154, 122, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MINIMUM_LIQUIDITY", abi = "MINIMUM_LIQUIDITY()")]
    pub struct MinimumLiquidityCall;
    #[doc = "Container type for all input parameters for the `PERMIT_TYPEHASH`function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(address)` and selector `[137, 175, 203, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address)")]
    pub struct BurnCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `getReserves`function with signature `getReserves()` and selector `[9, 2, 241, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReserves", abi = "getReserves()")]
    pub struct GetReservesCall;
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,address)` and selector `[72, 92, 201, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `kLast`function with signature `kLast()` and selector `[116, 100, 252, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "kLast", abi = "kLast()")]
    pub struct KlastCall;
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address)` and selector `[106, 98, 120, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address)")]
    pub struct MintCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `permit`function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `price0CumulativeLast`function with signature `price0CumulativeLast()` and selector `[89, 9, 192, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "price0CumulativeLast", abi = "price0CumulativeLast()")]
    pub struct Price0CumulativeLastCall;
    #[doc = "Container type for all input parameters for the `price1CumulativeLast`function with signature `price1CumulativeLast()` and selector `[90, 61, 84, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "price1CumulativeLast", abi = "price1CumulativeLast()")]
    pub struct Price1CumulativeLastCall;
    #[doc = "Container type for all input parameters for the `skim`function with signature `skim(address)` and selector `[188, 37, 207, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "skim", abi = "skim(address)")]
    pub struct SkimCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `swap`function with signature `swap(uint256,uint256,address,bytes)` and selector `[2, 44, 13, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "swap", abi = "swap(uint256,uint256,address,bytes)")]
    pub struct SwapCall {
        pub amount_0_out: ethers::core::types::U256,
        pub amount_1_out: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `sync`function with signature `sync()` and selector `[255, 246, 202, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sync", abi = "sync()")]
    pub struct SyncCall;
    #[doc = "Container type for all input parameters for the `token0`function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1`function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV2PairCalls {
        DomainSeparator(DomainSeparatorCall),
        MinimumLiquidity(MinimumLiquidityCall),
        PermitTypehash(PermitTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Factory(FactoryCall),
        GetReserves(GetReservesCall),
        Initialize(InitializeCall),
        Klast(KlastCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Price0CumulativeLast(Price0CumulativeLastCall),
        Price1CumulativeLast(Price1CumulativeLastCall),
        Skim(SkimCall),
        Swap(SwapCall),
        Symbol(SymbolCall),
        Sync(SyncCall),
        Token0(Token0Call),
        Token1(Token1Call),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV2PairCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <MinimumLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::MinimumLiquidity(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2PairCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::GetReserves(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <KlastCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Klast(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2PairCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2PairCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <Price0CumulativeLastCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Price0CumulativeLast(decoded));
            }
            if let Ok(decoded) =
                <Price1CumulativeLastCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Price1CumulativeLast(decoded));
            }
            if let Ok(decoded) = <SkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2PairCalls::Skim(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2PairCalls::Swap(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Symbol(decoded));
            }
            if let Ok(decoded) = <SyncCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2PairCalls::Sync(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Token1(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2PairCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV2PairCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV2PairCalls::DomainSeparator(element) => element.encode(),
                UniswapV2PairCalls::MinimumLiquidity(element) => element.encode(),
                UniswapV2PairCalls::PermitTypehash(element) => element.encode(),
                UniswapV2PairCalls::Allowance(element) => element.encode(),
                UniswapV2PairCalls::Approve(element) => element.encode(),
                UniswapV2PairCalls::BalanceOf(element) => element.encode(),
                UniswapV2PairCalls::Burn(element) => element.encode(),
                UniswapV2PairCalls::Decimals(element) => element.encode(),
                UniswapV2PairCalls::Factory(element) => element.encode(),
                UniswapV2PairCalls::GetReserves(element) => element.encode(),
                UniswapV2PairCalls::Initialize(element) => element.encode(),
                UniswapV2PairCalls::Klast(element) => element.encode(),
                UniswapV2PairCalls::Mint(element) => element.encode(),
                UniswapV2PairCalls::Name(element) => element.encode(),
                UniswapV2PairCalls::Nonces(element) => element.encode(),
                UniswapV2PairCalls::Permit(element) => element.encode(),
                UniswapV2PairCalls::Price0CumulativeLast(element) => element.encode(),
                UniswapV2PairCalls::Price1CumulativeLast(element) => element.encode(),
                UniswapV2PairCalls::Skim(element) => element.encode(),
                UniswapV2PairCalls::Swap(element) => element.encode(),
                UniswapV2PairCalls::Symbol(element) => element.encode(),
                UniswapV2PairCalls::Sync(element) => element.encode(),
                UniswapV2PairCalls::Token0(element) => element.encode(),
                UniswapV2PairCalls::Token1(element) => element.encode(),
                UniswapV2PairCalls::TotalSupply(element) => element.encode(),
                UniswapV2PairCalls::Transfer(element) => element.encode(),
                UniswapV2PairCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV2PairCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV2PairCalls::DomainSeparator(element) => element.fmt(f),
                UniswapV2PairCalls::MinimumLiquidity(element) => element.fmt(f),
                UniswapV2PairCalls::PermitTypehash(element) => element.fmt(f),
                UniswapV2PairCalls::Allowance(element) => element.fmt(f),
                UniswapV2PairCalls::Approve(element) => element.fmt(f),
                UniswapV2PairCalls::BalanceOf(element) => element.fmt(f),
                UniswapV2PairCalls::Burn(element) => element.fmt(f),
                UniswapV2PairCalls::Decimals(element) => element.fmt(f),
                UniswapV2PairCalls::Factory(element) => element.fmt(f),
                UniswapV2PairCalls::GetReserves(element) => element.fmt(f),
                UniswapV2PairCalls::Initialize(element) => element.fmt(f),
                UniswapV2PairCalls::Klast(element) => element.fmt(f),
                UniswapV2PairCalls::Mint(element) => element.fmt(f),
                UniswapV2PairCalls::Name(element) => element.fmt(f),
                UniswapV2PairCalls::Nonces(element) => element.fmt(f),
                UniswapV2PairCalls::Permit(element) => element.fmt(f),
                UniswapV2PairCalls::Price0CumulativeLast(element) => element.fmt(f),
                UniswapV2PairCalls::Price1CumulativeLast(element) => element.fmt(f),
                UniswapV2PairCalls::Skim(element) => element.fmt(f),
                UniswapV2PairCalls::Swap(element) => element.fmt(f),
                UniswapV2PairCalls::Symbol(element) => element.fmt(f),
                UniswapV2PairCalls::Sync(element) => element.fmt(f),
                UniswapV2PairCalls::Token0(element) => element.fmt(f),
                UniswapV2PairCalls::Token1(element) => element.fmt(f),
                UniswapV2PairCalls::TotalSupply(element) => element.fmt(f),
                UniswapV2PairCalls::Transfer(element) => element.fmt(f),
                UniswapV2PairCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for UniswapV2PairCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            UniswapV2PairCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<MinimumLiquidityCall> for UniswapV2PairCalls {
        fn from(var: MinimumLiquidityCall) -> Self {
            UniswapV2PairCalls::MinimumLiquidity(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for UniswapV2PairCalls {
        fn from(var: PermitTypehashCall) -> Self {
            UniswapV2PairCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for UniswapV2PairCalls {
        fn from(var: AllowanceCall) -> Self {
            UniswapV2PairCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for UniswapV2PairCalls {
        fn from(var: ApproveCall) -> Self {
            UniswapV2PairCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for UniswapV2PairCalls {
        fn from(var: BalanceOfCall) -> Self {
            UniswapV2PairCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for UniswapV2PairCalls {
        fn from(var: BurnCall) -> Self {
            UniswapV2PairCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for UniswapV2PairCalls {
        fn from(var: DecimalsCall) -> Self {
            UniswapV2PairCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for UniswapV2PairCalls {
        fn from(var: FactoryCall) -> Self {
            UniswapV2PairCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetReservesCall> for UniswapV2PairCalls {
        fn from(var: GetReservesCall) -> Self {
            UniswapV2PairCalls::GetReserves(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for UniswapV2PairCalls {
        fn from(var: InitializeCall) -> Self {
            UniswapV2PairCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<KlastCall> for UniswapV2PairCalls {
        fn from(var: KlastCall) -> Self {
            UniswapV2PairCalls::Klast(var)
        }
    }
    impl ::std::convert::From<MintCall> for UniswapV2PairCalls {
        fn from(var: MintCall) -> Self {
            UniswapV2PairCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for UniswapV2PairCalls {
        fn from(var: NameCall) -> Self {
            UniswapV2PairCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for UniswapV2PairCalls {
        fn from(var: NoncesCall) -> Self {
            UniswapV2PairCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PermitCall> for UniswapV2PairCalls {
        fn from(var: PermitCall) -> Self {
            UniswapV2PairCalls::Permit(var)
        }
    }
    impl ::std::convert::From<Price0CumulativeLastCall> for UniswapV2PairCalls {
        fn from(var: Price0CumulativeLastCall) -> Self {
            UniswapV2PairCalls::Price0CumulativeLast(var)
        }
    }
    impl ::std::convert::From<Price1CumulativeLastCall> for UniswapV2PairCalls {
        fn from(var: Price1CumulativeLastCall) -> Self {
            UniswapV2PairCalls::Price1CumulativeLast(var)
        }
    }
    impl ::std::convert::From<SkimCall> for UniswapV2PairCalls {
        fn from(var: SkimCall) -> Self {
            UniswapV2PairCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SwapCall> for UniswapV2PairCalls {
        fn from(var: SwapCall) -> Self {
            UniswapV2PairCalls::Swap(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for UniswapV2PairCalls {
        fn from(var: SymbolCall) -> Self {
            UniswapV2PairCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<SyncCall> for UniswapV2PairCalls {
        fn from(var: SyncCall) -> Self {
            UniswapV2PairCalls::Sync(var)
        }
    }
    impl ::std::convert::From<Token0Call> for UniswapV2PairCalls {
        fn from(var: Token0Call) -> Self {
            UniswapV2PairCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for UniswapV2PairCalls {
        fn from(var: Token1Call) -> Self {
            UniswapV2PairCalls::Token1(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for UniswapV2PairCalls {
        fn from(var: TotalSupplyCall) -> Self {
            UniswapV2PairCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for UniswapV2PairCalls {
        fn from(var: TransferCall) -> Self {
            UniswapV2PairCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for UniswapV2PairCalls {
        fn from(var: TransferFromCall) -> Self {
            UniswapV2PairCalls::TransferFrom(var)
        }
    }
}
