pragma solidity ^0.8.0;

import "@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol";

contract ArbitrageSwap {
	constructor() {}

	function swap(IUniswapV2Pair pool_a, IUniswapV2Pair pool_b, uint amount) public {
		pool_a.swap(amount, amount, address(pool_b), "");
		pool_b.swap(amount, amount, address(msg.sender), "");
	}
}