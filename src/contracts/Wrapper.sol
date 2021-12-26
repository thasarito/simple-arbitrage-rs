// SPDX-License-Identifier: Unlicense
pragma solidity 0.8.10;

import '@openzeppelin/token/ERC20/IERC20.sol';

contract Wrapper {
	IERC20 token;
	constructor(address _token) {
		token = IERC20(_token);
	}

	function ownerOf(address _owner) public view returns(uint) {
		uint balance = token.balanceOf(_owner);
		return balance;
	}
}
