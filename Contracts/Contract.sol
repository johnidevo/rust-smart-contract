
//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.15;

contract Addition{

	int public x;

	function add(int a, int b) public {
		x = a + b;
	}
}