
//use primitive_types::U256;

impl Opcode {
	pub fn describe(&self) {
		match self {
			
			Opcode::CALLDATASIZE(line) => println!("0x{:x}\tCALLDATASIZE\t---Halts execution", line),
			Opcode::CALLDATALOAD(line) => println!("0x{:x}\tCALLDATALOAD\t---Halts execution", line),
			
			Opcode::MLOAD(line) => println!("0x{:x}\tPRINT\t---Halts execution", line),
			Opcode::MSTORE(line) => println!("0x{:x}\tJUMPI\t---Halts execution", line),
			Opcode::MSTORE8(line) => println!("0x{:x}\tSLT\t---Addition operation", line),
			
			Opcode::PRINT(line) => println!("0x{:x}\tPRINT\t---Halts execution", line),
			Opcode::JUMPI(line) => println!("0x{:x}\tJUMPI\t---Halts execution", line),
			Opcode::SLT(line) => println!("0x{:x}\tSLT\t---Addition operation", line),
			
			Opcode::JUMP(line) => println!("0x{:x}\tJUMP\tHalts execution", line),
			
			Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
			Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
			Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
			Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
			Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
			_ => println!("Unknown opcode")
		}
	}
}

#[derive(Debug)]
pub enum Opcode {

	CALLDATASIZE(usize), // 0x00
	CALLDATALOAD(usize), // 0x01
	
	MLOAD(usize), // 0x00
	MSTORE(usize), // 0x01
	MSTORE8(usize), // 0x02
	
	PRINT(usize), // 0x00
	
	JUMP(usize), // 0x01
	JUMPI(usize), // 0x01
	SLT(usize), // 0x02
	
	STOP(usize), // 0x00
	ADD(usize), // 0x01
	MUL(usize), // 0x02
	
	// Push operations
	/*
	PUSH1(usize, U256), // 0x60
	PUSH2(usize, U256), // 0x61
	PUSH32(usize, U256),
	*/
	
	PUSH1(usize, u8), // 0x60
	PUSH2(usize, u8, u8), // 0x61
	PUSH32(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 
	// test commit mesage terminal
	EOF,
}
