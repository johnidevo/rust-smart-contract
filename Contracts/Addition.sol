//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.15;

contract Addition{

	int public x;
    
    function add() public {
        int a = 5;
        int b = 6;
    	x = a + b;
   	}
}