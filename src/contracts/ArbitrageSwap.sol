// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import "@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol";

interface IWETH9 {
    function deposit() external payable;

    function withdraw(uint256 wad) external;
}

contract ArbitrageSwap {
	IWETH9 weth;
	constructor(address payable _weth) {
		weth = IWETH9(_weth);
	}

	function swap(IUniswapV2Pair pool_a, IUniswapV2Pair pool_b, uint intermediate_amount, uint profit) public payable {
		weth.deposit{value: msg.value}();
		pool_a.swap(msg.value, intermediate_amount, address(pool_b), "");
		pool_b.swap(intermediate_amount, profit, address(msg.sender), "");
		weth.withdraw(profit);
		payable(msg.sender).transfer(profit);
	}
}