// SPDX-License-Identifier: Unlicense
pragma solidity 0.8.10;

import "ds-test/test.sol";
import '@openzeppelin/token/ERC20/ERC20.sol';
import "./Wrapper.sol";

contract TestWrapper is DSTest {
    Wrapper c;
    function setUp() public {
        ERC20 t = new ERC20("TSRT", "Thasarito");
        c = new Wrapper(address(t));
    }

    function test_ownerOf(address _owner, uint amount) public {
        uint balance = c.ownerOf(_owner);

        emit log_named_uint('balance', balance);
        emit log('\n');
        emit log_named_uint('amount', amount);
        emit log('\n');
        assertEq(balance, 0);
    }
}
