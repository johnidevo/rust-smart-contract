use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::error::Error;
//use std::path::Path;
//std::process::exit();

use primitive_types::U256;

use crate::params::params::InputParameters;
use crate::evm::opcode::Opcode;
use crate::evm::memory::Memory;


pub fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..(s.len()-1))
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i+2], 16))
		.collect()
}

pub struct Vm {
	pub code: Vec<u8>, // smart contract code
	pub pc: usize, // current instruction
	pub stack: Vec<U256>,
	// detect if code ended.
	pub at_end: bool,
	pub mem: Memory,
	// Parameters received in the message
	pub input_data: InputParameters,
}

impl Vm {
	pub fn new_from_file(filename: &str, input_data: InputParameters) -> Result<Vm, Box<dyn Error>> {
		//println!("{}", Path::new("./Artifacts/evm/mod.rs").exists());
		//println!("{}", filename);
		let mut file = File::open(filename).expect("File not found");
		let mut buffer = String::new();
		file.read_to_string(&mut buffer).expect("Error while reading file");
		
		/*
		let mut f = File::open(filename)?;
		let mut buffer = String::new();
		f.read_to_string(&mut buffer)?;
		*/
		
		let code = decode(&buffer)?;
		
		for b in &code {
			println!("0x{:x}", b) 
		}
		println!("{}", buffer);
		
		Ok(Vm { 
			code: code, 
			pc: 0, 
			stack: Vec::new(), 
			mem: Memory::new(), 
			input_data, 
			at_end: false
		})
		//Ok(Vm { code: code, pc: 0, stack: Vec::new(), at_end: false})
		//Ok(Vm { code: code, pc: 0, stack: Vec::new()})
	}

	// decoding
	pub fn next(&mut self) -> Option<Opcode> {
		
		println!("{:?}", self.code);
		
		if self.pc >= self.code.len() {
			return Some(Opcode::EOF);
		}

		//std::process::exit(self.pc.try_into().unwrap());
		let addr = self.pc;
		match self.code[addr] {
			0x00 => { self.pc += 1; Some(Opcode::STOP(addr)) }, /*_jj32_0_jjjj33_Halts execution*/
			0x01 => { self.pc += 1; Some(Opcode::ADD(addr)) }, /*_jj32_3_jjjj33_Addition operation*/
			0x02 => { self.pc += 1; Some(Opcode::MUL(addr)) }, /*_jj32_5_jjjj33_Multiplication operation*/
			0x03 => { self.pc += 1; Some(Opcode::SUB(addr)) }, /*_jj32_3_jjjj33_Subtraction operation*/
			0x04 => { self.pc += 1; Some(Opcode::DIV(addr)) }, /*_jj32_5_jjjj33_Integer division operation*/
			0x05 => { self.pc += 1; Some(Opcode::SDIV(addr)) }, /*_jj32_5_jjjj33_Signed integer division operation (truncated)*/
			0x06 => { self.pc += 1; Some(Opcode::MOD(addr)) }, /*_jj32_5_jjjj33_Modulo remainder operation*/
			0x07 => { self.pc += 1; Some(Opcode::SMOD(addr)) }, /*_jj32_5_jjjj33_Signed modulo remainder operation*/
			0x08 => { self.pc += 1; Some(Opcode::ADDMOD(addr)) }, /*_jj32_8_jjjj33_Modulo addition operation*/
			0x09 => { self.pc += 1; Some(Opcode::MULMOD(addr)) }, /*_jj32_8_jjjj33_Modulo multiplication operation*/
			0x0a => { self.pc += 1; Some(Opcode::EXP(addr)) }, /*_jj32_10_jjjj33_Exponential operation*/
			0x0b => { self.pc += 1; Some(Opcode::SIGNEXTEND(addr)) }, /*_jj32_5_jjjj33_Extend length of two’s complement signed integer*/
			0x10 => { self.pc += 1; Some(Opcode::LT(addr)) }, /*_jj32_3_jjjj33_Less-than comparison*/
			0x11 => { self.pc += 1; Some(Opcode::GT(addr)) }, /*_jj32_3_jjjj33_Greater-than comparison*/
			0x12 => { self.pc += 1; Some(Opcode::SLT(addr)) }, /*_jj32_3_jjjj33_Signed less-than comparison*/
			0x13 => { self.pc += 1; Some(Opcode::SGT(addr)) }, /*_jj32_3_jjjj33_Signed greater-than comparison*/
			0x14 => { self.pc += 1; Some(Opcode::EQ(addr)) }, /*_jj32_3_jjjj33_Equality comparison*/
			0x15 => { self.pc += 1; Some(Opcode::ISZERO(addr)) }, /*_jj32_3_jjjj33_Simple not operator*/
			0x16 => { self.pc += 1; Some(Opcode::AND(addr)) }, /*_jj32_3_jjjj33_Bitwise AND operation*/
			0x17 => { self.pc += 1; Some(Opcode::OR(addr)) }, /*_jj32_3_jjjj33_Bitwise OR operation*/
			0x18 => { self.pc += 1; Some(Opcode::XOR(addr)) }, /*_jj32_3_jjjj33_Bitwise XOR operation*/
			0x19 => { self.pc += 1; Some(Opcode::NOT(addr)) }, /*_jj32_3_jjjj33_Bitwise NOT operation*/
			0x1a => { self.pc += 1; Some(Opcode::BYTE(addr)) }, /*_jj32_3_jjjj33_Retrieve single byte from word*/
			0x20 => { self.pc += 1; Some(Opcode::SHA3(addr)) }, /*_jj32_30_jjjj33_Compute Keccak-256 hash*/
			0x30 => { self.pc += 1; Some(Opcode::ADDRESS(addr)) }, /*_jj32_2_jjjj33_Get address of currently executing account*/
			0x31 => { self.pc += 1; Some(Opcode::BALANCE(addr)) }, /*_jj32_20_jjjj33_Get balance of the given account*/
			0x32 => { self.pc += 1; Some(Opcode::ORIGIN(addr)) }, /*_jj32_2_jjjj33_Get execution origination address*/
			0x33 => { self.pc += 1; Some(Opcode::CALLER(addr)) }, /*_jj32_2_jjjj33_Get caller address*/
			0x34 => { self.pc += 1; Some(Opcode::CALLVALUE(addr)) }, /*_jj32_2_jjjj33_Get deposited value by the instruction/transaction responsible for this execution*/
			0x35 => { self.pc += 1; Some(Opcode::CALLDATALOAD(addr)) }, /*_jj32_3_jjjj33_Get input data of current environment*/
			0x36 => { self.pc += 1; Some(Opcode::CALLDATASIZE(addr)) }, /*_jj32_2_jjjj33_Get size of input data in current environment*/
			0x37 => { self.pc += 1; Some(Opcode::CALLDATACOPY(addr)) }, /*_jj32_3_jjjj33_Copy input data in current environment to memory*/
			0x38 => { self.pc += 1; Some(Opcode::CODESIZE(addr)) }, /*_jj32_2_jjjj33_Get size of code running in current environment*/
			0x39 => { self.pc += 1; Some(Opcode::CODECOPY(addr)) }, /*_jj32_3_jjjj33_Copy code running in current environment to memory*/
			0x3a => { self.pc += 1; Some(Opcode::GASPRICE(addr)) }, /*_jj32_2_jjjj33_Get price of gas in current environment*/
			0x3b => { self.pc += 1; Some(Opcode::EXTCODESIZE(addr)) }, /*_jj32_20_jjjj33_Get size of an account’s code*/
			0x3c => { self.pc += 1; Some(Opcode::EXTCODECOPY(addr)) }, /*_jj32_20_jjjj33_Copy an account’s code to memory*/
			0x40 => { self.pc += 1; Some(Opcode::BLOCKHASH(addr)) }, /*_jj32_20_jjjj33_Get the hash of one of the 256 most recent complete blocks*/
			0x41 => { self.pc += 1; Some(Opcode::COINBASE(addr)) }, /*_jj32_2_jjjj33_Get the block’s beneficiary address*/
			0x42 => { self.pc += 1; Some(Opcode::TIMESTAMP(addr)) }, /*_jj32_2_jjjj33_Get the block’s timestamp*/
			0x43 => { self.pc += 1; Some(Opcode::NUMBER(addr)) }, /*_jj32_2_jjjj33_Get the block’s number*/
			0x44 => { self.pc += 1; Some(Opcode::DIFFICULTY(addr)) }, /*_jj32_2_jjjj33_Get the block’s difficulty*/
			0x45 => { self.pc += 1; Some(Opcode::GASLIMIT(addr)) }, /*_jj32_2_jjjj33_Get the block’s gas limit*/
			0x50 => { self.pc += 1; Some(Opcode::POP(addr)) }, /*_jj32_2_jjjj33_Remove item from stack*/
			0x51 => { self.pc += 1; Some(Opcode::MLOAD(addr)) }, /*_jj32_3_jjjj33_Load word from memory*/
			0x52 => { self.pc += 1; Some(Opcode::MSTORE(addr)) }, /*_jj32_3_jjjj33_Save word to memory*/
			0x53 => { self.pc += 1; Some(Opcode::MSTORE8(addr)) }, /*_jj32_3_jjjj33_Save byte to memory*/
			0x54 => { self.pc += 1; Some(Opcode::SLOAD(addr)) }, /*_jj32_50_jjjj33_Load word from storage*/
			0x55 => { self.pc += 1; Some(Opcode::SSTORE(addr)) }, /*_jj32_5000_jjjj33_Save word to storage*/
			0x56 => { self.pc += 1; Some(Opcode::JUMP(addr)) }, /*_jj32_8_jjjj33_Alter the program counter*/
			0x57 => { self.pc += 1; Some(Opcode::JUMPI(addr)) }, /*_jj32_10_jjjj33_Conditionally alter the program counter*/
			0x58 => { self.pc += 1; Some(Opcode::PC(addr)) }, /*_jj32_2_jjjj33_Get the value of the program counter prior to the increment corresponding to this instruction*/
			0x59 => { self.pc += 1; Some(Opcode::MSIZE(addr)) }, /*_jj32_2_jjjj33_Get the size of active memory in bytes*/
			0x5a => { self.pc += 1; Some(Opcode::GAS(addr)) }, /*_jj32_2_jjjj33_Get the amount of available gas, including the corresponding reduction for the cost of this instruction*/
			0x5b => { self.pc += 1; Some(Opcode::JUMPDEST(addr)) }, /*_jj32_1_jjjj33_Mark a valid destination for jumps*/
			0x60 => { 
				let value = self.code[self.pc+1];
				self.pc += 2;
				Some(Opcode::PUSH1(addr, value)) 
			}, /*_jj32_3_jjjj33_Place 1 byte item on stack*/
			0x61 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				self.pc += 3;
				Some(Opcode::PUSH2(addr, value0, value1)) 
			}, /*_jj32_3_jjjj33_Place 2 byte item on stack*/
			0x62 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				self.pc += 4;
				Some(Opcode::PUSH3(addr, value0, value1, value2)) 
			}, /*_jj32_3_jjjj33_Place 3 byte item on stack*/
			0x63 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				let value3 = self.code[self.pc+4];
				self.pc += 5;
				Some(Opcode::PUSH4(addr, value0, value1, value2, value3)) 
			}, /*_jj32_3_jjjj33_Place 4 byte item on stack*/
			0x64 => { 				
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				let value3 = self.code[self.pc+4];
				let value4 = self.code[self.pc+5];
				self.pc += 6;
				Some(Opcode::PUSH5(addr, value0, value1, value2, value3, value4)) 
			}, /*_jj32_3_jjjj33_Place 5 byte item on stack*/
			0x65 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				let value3 = self.code[self.pc+4];
				let value4 = self.code[self.pc+5];
				let value5 = self.code[self.pc+6];
				self.pc += 7;
				Some(Opcode::PUSH6(addr, value0, value1, value2, value3, value4, value5)) 
			}, /*_jj32_3_jjjj33_Place 6 byte item on stack*/
			0x66 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				let value3 = self.code[self.pc+4];
				let value4 = self.code[self.pc+5];
				let value5 = self.code[self.pc+6];
				let value6 = self.code[self.pc+7];
				self.pc += 8;
				Some(Opcode::PUSH7(addr, value0, value1, value2, value3, value4, value5, value6)) 
			}, /*_jj32_3_jjjj33_Place 7 byte item on stack*/
			0x67 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				let value3 = self.code[self.pc+4];
				let value4 = self.code[self.pc+5];
				let value5 = self.code[self.pc+6];
				let value6 = self.code[self.pc+7];
				let value7 = self.code[self.pc+8];
				self.pc += 9;
				Some(Opcode::PUSH8(addr, value0, value1, value2, value3, value4, value5, value6, value7)) 
			}, /*_jj32_3_jjjj33_Place 8 byte item on stack*/
			0x68 => { 
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				let value2 = self.code[self.pc+3];
				let value3 = self.code[self.pc+4];
				let value4 = self.code[self.pc+5];
				let value5 = self.code[self.pc+6];
				let value6 = self.code[self.pc+7];
				let value7 = self.code[self.pc+8];
				let value8 = self.code[self.pc+9];
				self.pc += 10;
				Some(Opcode::PUSH9(addr, value0, value1, value2, value3, value4, value5, value6, value7, value8)) 
			}, /*_jj32_3_jjjj33_Place 9 byte item on stack*/
			0x69 => { self.pc += 1; Some(Opcode::PUSH10(addr)) }, /*_jj32_3_jjjj33_Place 10 byte item on stack*/
			0x6a => { self.pc += 1; Some(Opcode::PUSH11(addr)) }, /*_jj32_3_jjjj33_Place 11 byte item on stack*/
			0x6b => { self.pc += 1; Some(Opcode::PUSH12(addr)) }, /*_jj32_3_jjjj33_Place 12 byte item on stack*/
			0x6c => { self.pc += 1; Some(Opcode::PUSH13(addr)) }, /*_jj32_3_jjjj33_Place 13 byte item on stack*/
			0x6d => { self.pc += 1; Some(Opcode::PUSH14(addr)) }, /*_jj32_3_jjjj33_Place 14 byte item on stack*/
			0x6e => { self.pc += 1; Some(Opcode::PUSH15(addr)) }, /*_jj32_3_jjjj33_Place 15 byte item on stack*/
			0x6f => { self.pc += 1; Some(Opcode::PUSH16(addr)) }, /*_jj32_3_jjjj33_Place 16 byte item on stack*/
			0x70 => { self.pc += 1; Some(Opcode::PUSH17(addr)) }, /*_jj32_3_jjjj33_Place 17 byte item on stack*/
			0x71 => { self.pc += 1; Some(Opcode::PUSH18(addr)) }, /*_jj32_3_jjjj33_Place 18 byte item on stack*/
			0x72 => { self.pc += 1; Some(Opcode::PUSH19(addr)) }, /*_jj32_3_jjjj33_Place 19 byte item on stack*/
			0x73 => { self.pc += 1; Some(Opcode::PUSH20(addr)) }, /*_jj32_3_jjjj33_Place 20 byte item on stack*/
			0x74 => { self.pc += 1; Some(Opcode::PUSH21(addr)) }, /*_jj32_3_jjjj33_Place 21 byte item on stack*/
			0x75 => { self.pc += 1; Some(Opcode::PUSH22(addr)) }, /*_jj32_3_jjjj33_Place 22 byte item on stack*/
			0x76 => { self.pc += 1; Some(Opcode::PUSH23(addr)) }, /*_jj32_3_jjjj33_Place 23 byte item on stack*/
			0x77 => { self.pc += 1; Some(Opcode::PUSH24(addr)) }, /*_jj32_3_jjjj33_Place 24 byte item on stack*/
			0x78 => { self.pc += 1; Some(Opcode::PUSH25(addr)) }, /*_jj32_3_jjjj33_Place 25 byte item on stack*/
			0x79 => { self.pc += 1; Some(Opcode::PUSH26(addr)) }, /*_jj32_3_jjjj33_Place 26 byte item on stack*/
			0x7a => { self.pc += 1; Some(Opcode::PUSH27(addr)) }, /*_jj32_3_jjjj33_Place 27 byte item on stack*/
			0x7b => { self.pc += 1; Some(Opcode::PUSH28(addr)) }, /*_jj32_3_jjjj33_Place 28 byte item on stack*/
			0x7c => { self.pc += 1; Some(Opcode::PUSH29(addr)) }, /*_jj32_3_jjjj33_Place 29 byte item on stack*/
			0x7d => { self.pc += 1; Some(Opcode::PUSH30(addr)) }, /*_jj32_3_jjjj33_Place 30 byte item on stack*/
			0x7e => { self.pc += 1; Some(Opcode::PUSH31(addr)) }, /*_jj32_3_jjjj33_Place 31 byte item on stack*/
			0x7f => { self.pc += 1; Some(Opcode::PUSH32(addr)) }, /*_jj32_3_jjjj33_Place 32 byte (full word) item on stack*/
			0x80 => { self.pc += 1; Some(Opcode::DUP1(addr)) }, /*_jj32_3_jjjj33_Duplicate 1st stack item*/
			0x81 => { self.pc += 1; Some(Opcode::DUP2(addr)) }, /*_jj32_3_jjjj33_Duplicate 2nd stack item*/
			0x82 => { self.pc += 1; Some(Opcode::DUP3(addr)) }, /*_jj32_3_jjjj33_Duplicate 3rd stack item*/
			0x83 => { self.pc += 1; Some(Opcode::DUP4(addr)) }, /*_jj32_3_jjjj33_Duplicate 4th stack item*/
			0x84 => { self.pc += 1; Some(Opcode::DUP5(addr)) }, /*_jj32_3_jjjj33_Duplicate 5th stack item*/
			0x85 => { self.pc += 1; Some(Opcode::DUP6(addr)) }, /*_jj32_3_jjjj33_Duplicate 6th stack item*/
			0x86 => { self.pc += 1; Some(Opcode::DUP7(addr)) }, /*_jj32_3_jjjj33_Duplicate 7th stack item*/
			0x87 => { self.pc += 1; Some(Opcode::DUP8(addr)) }, /*_jj32_3_jjjj33_Duplicate 8th stack item*/
			0x88 => { self.pc += 1; Some(Opcode::DUP9(addr)) }, /*_jj32_3_jjjj33_Duplicate 9th stack item*/
			0x89 => { self.pc += 1; Some(Opcode::DUP10(addr)) }, /*_jj32_3_jjjj33_Duplicate 10th stack item*/
			0x8a => { self.pc += 1; Some(Opcode::DUP11(addr)) }, /*_jj32_3_jjjj33_Duplicate 11th stack item*/
			0x8b => { self.pc += 1; Some(Opcode::DUP12(addr)) }, /*_jj32_3_jjjj33_Duplicate 12th stack item*/
			0x8c => { self.pc += 1; Some(Opcode::DUP13(addr)) }, /*_jj32_3_jjjj33_Duplicate 13th stack item*/
			0x8d => { self.pc += 1; Some(Opcode::DUP14(addr)) }, /*_jj32_3_jjjj33_Duplicate 14th stack item*/
			0x8e => { self.pc += 1; Some(Opcode::DUP15(addr)) }, /*_jj32_3_jjjj33_Duplicate 15th stack item*/
			0x8f => { self.pc += 1; Some(Opcode::DUP16(addr)) }, /*_jj32_3_jjjj33_Duplicate 16th stack item*/
			0x90 => { self.pc += 1; Some(Opcode::SWAP1(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 2nd stack items*/
			0x91 => { self.pc += 1; Some(Opcode::SWAP2(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 3rd stack items*/
			0x92 => { self.pc += 1; Some(Opcode::SWAP3(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 4th stack items*/
			0x93 => { self.pc += 1; Some(Opcode::SWAP4(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 5th stack items*/
			0x94 => { self.pc += 1; Some(Opcode::SWAP5(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 6th stack items*/
			0x95 => { self.pc += 1; Some(Opcode::SWAP6(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 7th stack items*/
			0x96 => { self.pc += 1; Some(Opcode::SWAP7(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 8th stack items*/
			0x97 => { self.pc += 1; Some(Opcode::SWAP8(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 9th stack items*/
			0x98 => { self.pc += 1; Some(Opcode::SWAP9(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 10th stack items*/
			0x99 => { self.pc += 1; Some(Opcode::SWAP10(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 11th stack items*/
			0x9a => { self.pc += 1; Some(Opcode::SWAP11(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 12th stack items*/
			0x9b => { self.pc += 1; Some(Opcode::SWAP12(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 13th stack items*/
			0x9c => { self.pc += 1; Some(Opcode::SWAP13(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 14th stack items*/
			0x9d => { self.pc += 1; Some(Opcode::SWAP14(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 15th stack items*/
			0x9e => { self.pc += 1; Some(Opcode::SWAP15(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 16th stack items*/
			0x9f => { self.pc += 1; Some(Opcode::SWAP16(addr)) }, /*_jj32_3_jjjj33_Exchange 1st and 17th stack items*/
			0xa0 => { self.pc += 1; Some(Opcode::LOG0(addr)) }, /*_jj32_375_jjjj33_Append log record with no topics*/
			0xa1 => { self.pc += 1; Some(Opcode::LOG1(addr)) }, /*_jj32_750_jjjj33_Append log record with one topic*/
			0xa2 => { self.pc += 1; Some(Opcode::LOG2(addr)) }, /*_jj32_1125_jjjj33_Append log record with two topics*/
			0xa3 => { self.pc += 1; Some(Opcode::LOG3(addr)) }, /*_jj32_1500_jjjj33_Append log record with three topics*/
			0xa4 => { self.pc += 1; Some(Opcode::LOG4(addr)) }, /*_jj32_1875_jjjj33_Append log record with four topics*/
			0xf0 => { self.pc += 1; Some(Opcode::CREATE(addr)) }, /*_jj32_32000_jjjj33_Create a new account with associated code*/
			0xf1 => { self.pc += 1; Some(Opcode::CALL(addr)) }, /*_jj32_40_jjjj33_Message-call into an account*/
			0xf2 => { self.pc += 1; Some(Opcode::CALLCODE(addr)) }, /*_jj32_40_jjjj33_Message-call into this account with alternative account’s code*/
			0xf3 => { self.pc += 1; Some(Opcode::RETURN(addr)) }, /*_jj32_0_jjjj33_Halt execution returning output data*/
			0xf4 => { self.pc += 1; Some(Opcode::DELEGATECALL(addr)) }, /*_jj32_40_jjjj33_Message-call into this account with an alternative account’s code, but persisting the current values for sender and value*/
			0xfe => { self.pc += 1; Some(Opcode::INVALID(addr)) }, /*_s_NaN_s_Designated invalid instruction*/
			0xff => { self.pc += 1; Some(Opcode::SELFDESTRUCT(addr)) }, /*_jj32_0_jjjj33_Halt execution and register account for later deletion*/

			/*
			 0x00 => {
				self.pc += 1;
				Some(Opcode::STOP(addr))
			},
			0x01 => {
				self.pc += 1;
				Some(Opcode::ADD(addr))
			},
			0x02 => {
				self.pc += 1;
				Some(Opcode::MUL(addr))
			},
			0x60 => {
				let value = self.code[self.pc+1];
				self.pc += 2;
				Some(Opcode::PUSH1(addr, value))
			},
			0x61 => {
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				self.pc += 3;
				Some(Opcode::PUSH2(addr, value0, value1))
			},
			*/
			_ => { self.pc += 1;  None}
		}
	}

	pub fn interpret(&mut self) {
		let maybe_op = self.next();

		// just for debugging
		match &maybe_op {
			Some(x) => x.describe(),
			None => {}
		}

		// The real execution
		match &maybe_op {
			Some(x) => {
				match x {
					Opcode::CALLDATASIZE(_) => {
						let size = self.input_data.size();
						self.stack.push(size);
					},
					Opcode::CALLDATALOAD(_) => {
						// This is a bit dirty. As first approximation, there is not
						// way we would have a size larger than 32 bits. Lets try it
						// and if it fails, it will panic (which is what I want)
						let idx = self.stack.pop().unwrap().as_u32() as usize;
						let data = self.input_data.get(idx);
						self.stack.push(data);
					},
					Opcode::MLOAD(_addr) => {
						let offset = self.stack.pop().unwrap();
						let loaded_value = self.mem.get_word(offset.as_u64() as usize);
						self.stack.push(loaded_value);
					},
					Opcode::MSTORE(addr) => {
						let offset = self.stack.pop().unwrap();
						let w = self.stack.pop().unwrap();
						self.mem.set_word(offset.as_u64() as usize, w);
					},
					Opcode::MSTORE8(_addr) => {
						// stored as big endian so we get the last byte
						let offset = self.stack.pop().unwrap();
						let b = self.stack.pop().unwrap().byte(31);
						self.mem.set_byte(offset.as_u64() as usize, b);
					},
					Opcode::JUMP(_addr) => {
						let jump_location = self.stack.pop().unwrap();
						self.pc = jump_location.as_u64() as usize;
					},
					Opcode::SLT(_addr) => {
						let lhs = self.stack.pop().unwrap();
						let rhs = self.stack.pop().unwrap();
						if lhs < rhs {
							self.stack.push(U256::from(0x01));
						} else {
							self.stack.push(U256::from(0x00));
						}
					},
					Opcode::JUMPI(_addr) => {
						let then_addr = self.stack.pop().unwrap();
						let cond = self.stack.pop().unwrap();
						if !cond.is_zero() {
							self.pc = then_addr.as_u64() as usize;
						} // else continue to next :)
					}
					Opcode::PRINT(_addr) => {
						// TODO this should be removed in release mode..
						let v = self.stack.pop().unwrap();
						let mut bytes = vec![0;32];
						v.to_big_endian(&mut bytes);
						println!("PRINT\t{:?}|", bytes)
					},
					Opcode::PUSH1(addr, value) => {
						// Value is u8, we need to push a u256 on the stack...
						//self.stack.push(value);
						self.stack.push(U256::from(*value));
					},
					Opcode::ADD(addr) => {
						// How to recover nicely? There is no meaning in recovering nicely here.
						// Just burn and crash if cannot pop.
						let v1 = self.stack.pop().unwrap();
						let v2 = self.stack.pop().unwrap();
						self.stack.push(v1 + v2);
					},
					_ => {
					// not implemented, just chill
					}
				}
			},
			None => {}
		}
  }
	
	
	pub fn get_new_size(&self, code: &Opcode) -> Option<usize> {
		match code {
			Opcode::MLOAD(_) | Opcode::MSTORE(_) => {
				Some(self.stack.last().unwrap().as_u64() as usize + 32)
			},
			Opcode::MSTORE8(_) => {
				Some(self.stack.last().unwrap().as_u64() as usize + 1)
			},
			_ => None  
		}
	}

	// see part 2 for print_stack
	pub fn print_stack(&self) {
		self.stack
			.iter()
			.enumerate()
			.rev()
			.for_each(|(i, x)| {
				let mut bytes = vec![0;32];
				x.to_big_endian(&mut bytes);
				println!("|{}:\t{:?}|", i, bytes)
			});
	}

	pub fn print_debug(&self) {
		println!("pc:{}\n", self.pc);
		println!("Stack:");
		self.print_stack();
	}
}
