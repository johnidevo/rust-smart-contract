(Part 1) by snoozetime (2018)
* …
* VM and showed a simple way of printing instructions from a smart contract binary file
* more about the memory model of the VM and how to understand the code compiled by solc.
* what kind of operations are possible using the stack


(Part 2) by snoozetime (2018)


VM specifications
* The VM is described in the yellow paper, part 9: Yellow paper
* The word size of the machine (and thus size of stack items 0 is 256-bits.
* 32 bytes word size is quite large. Yellow paper mentioned it is to facilitate the Keccak-256 hash scheme and elliptic-curve computations
* Stack-based architectures carry out their operation by pushing and popping items on a stack. A stack is just a LIFO (last in, first out) container. For example, a + b can be done by:
* pushing a on the stack
* pushing b on the stack
* pop a and pop b
* push a+b on the stack
* Memory is a random-access byte array. We read a word at a time (32 bytes).
* Unlike memory which is volatile, storage is non volatile and is maintained as part of the system state. This is where the fun happen. The data is persisted between transactions in this storage. It is stored under the contract’s account.


Fun with the stack
* the best way to model a stack is just to use a vector. Vec has push and pop operations. Unfortunately, the size of the stack items is 256 bits, which is not standard in Rust so I needed to use an external package (I don’t really want to implement 256-bits arithmetic myself…).
* part 1 implementation, our VM will become (struct)… And the factory function (new VM)...


Additions
* As mentioned above, an addition implemented with a stack can be with the following operations:
* PUSH left operand
* PUSH right operand
* POP 2 items from stack and add them
* PUSH result
* It can be easily translated to Ethereum instructions.
* PUSH1 left
* PUSH1 right
* ADD
* For example, if we want to add 2 and 5, the binary code would be: 0x60 0x02 0x60 0x05 0x01 PUSH 2 PUSH 5 ADD
* Stack:
* |0:         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128]|


Find the addition in our compile Ethereum contract
* solc --bin-runtime --overwrite --asm --optimize -o . contract.sol and examine the .evm file that was created.
* What happens here?
* 0xb is a shorthand for “PUSH 0xb” on the stack
* 0x0 is a shorthand for “PUSH 0x0” on the stack
* sstore will pop two values from the stack. The first one will be the key, the second one will be the value. It will then store Key=>Value in persistent storage.
* There are a lot of superfluous instructions here. Swap, pop and dup are just stack operations:
* [0] // int a [0, 0] // int b [0, 0, 5] // 5 [5, 0, 0] // a = 5 [5, 0] // pop
* Stack:
* |1:         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64]|
* |0:         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128]|
* Please type either c, s or q
* c
* 0x0         PUSH1   Place 1-byte item on the stack 0x80
* c
* 0x2         PUSH1   Place 1-byte item on the stack 0x40
* This map quite well to the .ecm file ;)


Flow control - If and Loops


(Part 3) by snoozetime (2018)


Looping and calling Functions in the EVM!
* Implement an interpreter in Rust


Compiling a loop
* Just by looking at the code and thinking the basic elements of a IF statement, we can guess how this will be compiled.
* First, we need to initialize i. Push 0 on the stack.
* Then, we need to compare with 0xA.
* Using the JUMPI instruction, we can either execute the loop body or skip it.
* We execute the loop body
* We increase i at the end of the loop body
* we jump back to the condition
* The missing block here is the JUMP back.


Testing in the VM


Interlude
* What could go wrong.


A quick peak at the compile code
* 0xef        JUMPDES         Destination
* 0xf0        PUSH1        Place 1-byte item on the stack 0x0
* 0xf2        PUSH1        Place 1-byte item on the stack 0x2a
* 0xf4        DUP3        Duplicate 3rd stack item.
* 0xf5        ADD        Addition operation
* 0xf6        DUP4        Duplicate 4th stack item.
* 0xf7        MUL        Multiplication operation
* 0xf8        SWAP1        Swap 1st and 2nd stack items.
* 0xf9        POP        Remove item from stack
* 0xfa        SWAP3        Swap 1st and 4th stack items.
* 0xfb        SWAP2        Swap 1st and 3rd stack items.
* 0xfc        POP        Remove item from stack
* 0xfd        POP        Remove item from stack
* 0xfe        JUMP         Jump to destination
* 0xff        JUMPDES         Destination
* 0x100        PUSH2        Place 2-bytes item on the stack 0x1 0xb
* 0x103        PUSH1        Place 1-byte item on the stack 0x1
* 0x105        PUSH1        Place 1-byte item on the stack 0x4
* 0x107        PUSH2        Place 2-bytes item on the stack 0x0 0xef
* 0x10a        JUMP         Jump to destination
* 0x10b        JUMPDES         Destination
* 0x10c        PUSH1        Place 1-byte item on the stack 0x0
* 0x10e        DUP2        Duplicate 2nd stack item.
* 0x10f        SWAP1        Swap 1st and 2nd stack items.
* 0x110        SSTORE        Save word to storage
* 0x111        POP        Remove item from stack
* 0x112        JUMP         Jump to destination
* 0x113        STOP        Halts execution
* No biggies here. The function starts at address 0x100. Let’s take a look at the stack during the execution.
* [0x10b]
* [0x10b, 0x01]
* [0x10b, 0x01, 0x04]
* [0x10b, 0x01, 0x04, 0xef]
* This is the stack before 0x10a (JUMP).
* Then, the store instruction is executed as expected.


After the stack, the memory
* In my previous posts about the Ethereum VM, I focused on very simple examples, using Value Types. These are variable that will always be passed by value according to the Solidity documentation.
* We saw that passing by value involved the stack. However, Reference types, which are types that do not always fit in 256 bit, should be store either in the memory or in the storage. As opposed to the storage, the memory does not persist between contract’s execution.
* Reference Types are:
* arrays
* struct
* mapping
* the default location for function parameters is memory and the default for local variables is storage. State variables are forced to have their location in storage.
* Well, what a bummer. If I want to exhibit the usage of memory in our compiled code, I’d need to use a more complex example.


(Part 4) by snoozetime (2018)


Solidity code
```
* contract Example {
*     struct Position {
*         address owner;
*         uint id;
*     }
*     
*     uint x; 
*     function takeOver() public {
*         Position memory p = Position(msg.sender, 0);
*         x = p.id;
*     }
* }
```
* Again a useless contract.
* I just want to exhibit the use of the memory without involving the storage too much
* What I want to understand here is:
* How a struct is defined in binary code
* How memory is used to store 


Specifications for memory 


Memory layout
* “The memory model is a simple word-addressed byte array.”
* The stack had two basic operations: push and pop.
* The memory is word addressed so we can retrieve data at specific addresses in the byte array. Inserting is also done wherever in the array.
* In solidity, the memory follows a specific layout (Doc). Four 32 bytes slots are reserved for solidity usage:
* 0x00 to 0x3f: Scratch space for hashing methods
* 0x40-0x5f: Free memory pointer
* 0x60-0x7f: Zero slot
* Solidity always places new objects at the free memory pointer and memory is never freed (this might change in the future).
* When creating an object in memory, we first need to take a look at the first available address. Then we can insert the object in memory at this address. The first available address, i.e. the free memory pointer, can be loaded with mload(0x40). This instruction will load a 32 bytes address.
Opcodes
* According to the yellow paper, the instructions that deal with the memory are:
* SHA3: Compute Keccak-256 hash
* CALLDATACOPY: Copy input data in current environment to memory
* CODECOPY: Copy code running in current environment to memory
* EXTCODECOPY: Copy an account’s code to memory
* MLOAD: Load word from memory
* MSTORE: Save word to memory
* MSTORE8: Save byte to memory
* CREATE: Create a new account with associated code
* CALL: Message-call into an account
* RETURN: Halt execution returning output data
* You can see that there is not arithmetic nor control flow involved in these instructions. These instructions are the domain of the stack.
* Something to note about memory is that the amount of gas to pay will grow with the memory usage.


Decompiling our compiled code
* As always, solc is a dear an annotate the .evm code for us. Let’s decompose it and show the current stack and memory.
* stack: [...]
* memory: ... |0x40 -> addr1 | .... | ....
* The memory just contains the first available address (addr1). The stack state is unknown.
* Now we have our answer as why only 2 times 32 bytes were allocated. It was to store the msg.sender and 0 that are the members of our structure. The address of p is addr1. The content of Position is known at compile-time so solc knows about many bytes are required for Position.
* We want to store p.id in x. Solidity will store x at address 0x0 in the storage. It will first load p.id from memory. The address of this field is just address of p + offset of field ID, which is addr1 + 0x20.


Emulation in Rust
* In order to avoid duplicating code too much, I refactored a bit the enumeration to accept directly U256 in PUSHX values.
* That’s much less complexity. I guess we could factored it more to get the numbers of byte to read for each PUSH instruction and group everything under one match case. For now it should be fine.
* By the way, saving the free memory pointer at 0x40 is from Solidity specification, not the EVM.
* A vector initial size is 0. When calling instructions such as MSTORE or MLOAD, a certain address is specified. If the address is out of range of the current memory, we need to resize the vector. In go-ethereum, this is done by resize up to the requested index + a certain amount of bytes. See here:
* https://github.com/ethereum/go-ethereum/blob/cab1cff11cbcd4ff60f1a149deb71ec87413b487/core/vm/memory_table.go
* https://github.com/ethereum/go-ethereum/blob/b66f793443f572082d24f115e706532a620ba3ee/core/vm/memory.go
* I will do something similar. Simply use a structure wrapping a Vec<u8>. Resize it if an instruction needs to access an address. For our tests, the following should do the trick: // memory.rs


(Part 5) by snoozetime (2018)
* But first of all what happens when an Ethereum peer receives a new message?


A look at Parity code
* An Ethereum client is not only a virtual machine. It deals with many more things such as:
* A peer-to-peer network
* A RPC server for client’s requests
* A consensus algorithm
* The actual blockchain implementation
* It is very interesting to explore but also a bit overwhelming. What I learned from there is enough for now. Basically when the node receives a message, it will:
* extract the parameters from the message
* Create a new VM (memory and stack empty) with the message parameters.
* Start executing the code in the VM from the first instruction
* . The message’s parameters are also injected in the VM. You can retrieve them using specials opcodes: CALLDATALOAD, CALLDATASIZE.


Dissecting a smart contract
* I am going to follow a compiled solidity contract from the beginning.
* Now, let’s take a look at this smart contract binary. We have:
* The first instruction mstore(0x40, 0x80) is actually storing the free memory pointer address in memory. jumpi(tag_1, lt(calldatasize, 0x4)) will move to tag_1 if the size of the input data is too small. If that’s the case, the contract will abort (revert).
* The snipped ends with a jump to tag_6, which is the body of the function multiply. The snippet of code here is just extracting data from the input parameters:
* callvalue will copy the Wei sent to this contract to the stack
* If callvalue is 0, the contract will revert, else it will jump to tag_4
* A bunch of instructions are done with calldataload to push a and b on the stack.
* There is still a missing piece though. How is the input data sent to the smart contract? How is it loaded? How do we map the value 0x165c4a16 to the correct function?


Input data in detail
* Phew, that’s a lot of assembly code. We encountered a few mysteries on our way:
* Why are we checking that input data size is less than 4?
* How do we find out the label for the function to execute?
* How do we get the function parameters?
* To summarize, the method ID is created by taking the first 4 bytes of the Keccak hash of the function’s signature. Then, the arguments are added to the string. In the case of our integer, they will be converted to their hexadecimal form, padded 32 bytes (256 bit, size of the EVM word). In the binary code, we check that the input data size if at least 4! This is because we need to get the method ID. Then, the method ID is extracted using some bytes operations.
* Now, take a look at the following image. It explains all our calls to CALLDATALOAD.
  

* First, we get 32 bytes from index 0x0 (CALLDATALOAD(0x0)). It includes our method ID and a bit of the first parameter so we need to extract only 4 bytes. This is done by dividing by (1 « (29*8)) and extracting the 4 first bytes with (AND 0xFFFFFFFF). I wonder why the code didn’t compile to a right shift only here. Maybe the optimized version is doing that. Then, we get the first parameter with CALLDATALOAD(0x4) and the second parameter with CALLDATALOAD(0x24). After that, it is business as usual!


Model the input data and CALLDATALOAD/CALLDATASIZE in our VM
* The input data is, as we saw, just a byte array that return words. The following can be used.
* Then, the VM itself needs to be modified to accept InputParameters as input data. We also need to add the opcodes and implementation of CALLDATALOAD and CALLDATASIZE.
* I am going to write EVM code in assembly that should read input data and execute it using my VM. Then, I’ll take a look at the persistent storage.
* Here I will create a simple EVM that can receive message from an HTTP POST request. The smart contract will be written in assembly code (and very simple obviously).
* With Rust, I will have to:
* Create a small HTTP server that can accept POST requests with JSON data
* Execute the smart contract with the input parameters contained in the JSON data
* Return the result to the client that sent the HTTP request
* In Assembly, I will create the following contract:
* add accepts two integers and add them together
* square accepts one integer and return its square


Crafting the smart contract
* 0x0         PUSH1   Place 1-byte item on the stack 0x80
* 0x2         PUSH1   Place 1-byte item on the stack 0x40
* Stack:
* |1:         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64]|
* |0:         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128]|
* We can try this without the HTTP interface using the existing code. Use this as input parameter and run the binary. It should finish with 0x04 at the top of the stack.
*  let input_data = params::InputParameters::new(vec![0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
* * 





References
* snoozetime. (2018). Ethereum Virtual Machine in Rust - Part 2: The stack. https://snoozetime.github.io/2018/11/14/ethereum-vm-2.html