//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.15;

contract Example {
    struct Position {
        address owner;
        uint id;
    }
    
    uint x; 
    function takeOver() public {
        Position memory p = Position(msg.sender, 0);
        x = p.id;
    }
}