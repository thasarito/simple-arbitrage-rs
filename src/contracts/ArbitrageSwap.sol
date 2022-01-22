// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import "@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol";
import "@openzeppelin/contracts/interfaces/IERC20.sol";

interface IWETH9 {
    function deposit() external payable;
    function transfer(address dst, uint wad) external returns (bool);
    function withdraw(uint256 wad) external;
}

contract ArbitrageSwap {
	IWETH9 weth;
	constructor(address payable _weth) {
		weth = IWETH9(_weth);
	}

	function swap(IUniswapV2Pair pool_a, IUniswapV2Pair pool_b, IERC20 token, uint intermediate_amount, uint profit) public payable {
		weth.deposit{value: msg.value}();
		(uint amount0, uint amount1) = pool_a.token0() == address(token) ? (intermediate_amount, uint(0)) : (uint(0), intermediate_amount);
		weth.transfer(address(pool_a), msg.value);
		pool_a.swap(amount0, amount1, address(pool_b), "");

		(uint amount0b, uint amount1b) = pool_b.token0() == address(token) ? (uint(0), profit) : (profit, uint(0));
		pool_b.swap(amount0b, amount1b, address(this), "");
		weth.transfer(msg.sender, profit);
		// weth.withdraw(profit);
		// payable(msg.sender).transfer(profit);
	}

	receive() external payable {}
}