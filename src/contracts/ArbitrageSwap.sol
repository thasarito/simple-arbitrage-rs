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
		(uint amount_token, uint amount_eth) = pool_a.token0() == address(token) ? (intermediate_amount, msg.value) : (msg.value, intermediate_amount);
		weth.deposit{value: amount_eth}();
		weth.transfer(address(pool_a), amount_eth);
		pool_a.swap(amount_token, 0, address(pool_b), "");

		// (uint amount_token_b, uint amount_eth_b) = pool_b.token0() == address(token) ? (amount_token, profit) : (profit, amount_token);
		// token.transfer(address(pool_b), amount_token_b);
		// pool_b.swap(0, amount_eth_b * 990 / 1000, address(msg.sender), "");
		// weth.withdraw(profit);
		// payable(msg.sender).transfer(profit);
	}
}