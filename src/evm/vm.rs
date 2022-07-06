use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::error::Error;
//use std::path::Path;
//use crypto_bigint::U256;

use primitive_types::U256;

use super::opcode::Opcode;

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
/*
		for b in &code {
			println!("0x{:x}", b) 
		}
		println!("{}", buffer);
*/
		Ok(Vm { code: code, pc: 0, stack: Vec::new(), at_end: false})
		//Ok(Vm { code: code, pc: 0, stack: Vec::new()})
	}

	// decoding
	pub fn next(&mut self) -> Option<Opcode> {
		if self.pc >= self.code.len() {
			return Some(Opcode::EOF);
		}

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

