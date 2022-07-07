
//use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::error::Error;
//use std::path::Path;
//std::process::exit();

use primitive_types::U256;

mod evm {
	pub mod vm {

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
	pub at_end: bool,
}

impl Vm {
	pub fn new_from_file(filename: &str) -> Result<Vm, Box<dyn Error>> {
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
		
		Ok(Vm { code: code, pc: 0, stack: Vec::new(), at_end: false})
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
									
									Opcode::MLOAD(_addr) => {
											let offset = self.stack.pop().unwrap();
											let loaded_value = self.mem.get_word(offset.as_u64() as usize);
											self.stack.push(loaded_value);
									},
									Opcode::MSTORE(_addr) => {
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

	}
	
	pub mod opcode {
impl Opcode {
	pub fn describe(&self) {
		match self {
			
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

	}
	pub mod memory {
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {

    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn resize(&mut self, new_size: usize) {
        if self.data.len() < new_size {
            self.data.resize(new_size, 0);
        }
    }

    // We only get words from the memory
    pub fn get_word(&self, addr: usize) -> U256 {
        // will panic if oob
        U256::from_big_endian(&self.data[addr..addr+32])
    }

    pub fn set_byte(&mut self, addr: usize, b: u8) {
        self.data[addr] = b;
    }

    pub fn set_word(&mut self, addr: usize, w: U256) {
        let mut bytes = vec![0; 32];
        w.to_big_endian(&mut bytes);

        for i in 0..bytes.len() {
            self.data[i+addr] = bytes[i];
        }
    }
}
	}
}


fn debug(vm: &mut Vm) {
	loop {
		match vm.next() {
			Some(Opcode::EOF) => break,
			//Opcode::EOF => break,
			Some(x) => x.describe(),
			//x => x.describe(),
			None => {}
		}
	}
}


fn create_vm(binary: Vec<u8>) -> Vm {
    Vm { code: binary, pc: 0, stack: Vec::new(), at_end: false}
}

fn vm_test() {
	// 1 + 5
	let binary = vec![0x60, 0x0f, 0x60, 0x01, 0x01, 0x00];
	let mut vm = create_vm(binary); //moved

	// execute three instructions.
	// push 0x0f
	vm.interpret();
	// push 0x01
	vm.interpret();
	// add
	vm.interpret();
	// halt
	vm.interpret();

	// Now make sure the stack size is 1 and contains 16.
	assert_eq!(1, vm.stack.len());
	assert_eq!(16, vm.stack[0].as_u32()); // this is panicking in case of overflow.
}

fn interpret() {//vm: &mut Vm) {
	println!("Infinite loop");
	/*
    while !vm.at_end {
        vm.interpret();
    }
    vm.print_stack();
	*/
}

fn debugger(vm: &mut Vm) {
	loop {
		if vm.at_end {
			break;
		}
		
		// Debugger.
		// c to continue
		// s to print stack
		// q to quit
		let mut input = String::new();
		io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

		match &*input {
			"c\n" => vm.interpret(),
			"s\n" => vm.print_debug(),
			"q\n" => break,
			_ => println!("Please type either c, s or q"), 
		}
	}
}

fn run() -> Result<(), Box<dyn Error>> {

	//let args: Vec<String> = env::args().collect();
	let function = "debugger"; //args[1].clone();
	let filename = "./Artifacts/04 Example/Example.bin-runtime"; //args[2].clone();

	println!("In file {}", filename);

	let mut vm = Vm::new_from_file(&filename)?;
	println!("Correctly loaded VM");

	match &*function {
		"debugger" => debugger(&mut vm),
		"debug" => debug(&mut vm),
		"run" => interpret(),
		"test" => vm_test(),
		_ => panic!("Expect either 'debug' or 'run' for first parameter")
	}

	Ok(())
}

fn main() {
    run().unwrap();
}
