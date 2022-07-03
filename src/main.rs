use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::error::Error;
//use std::path::Path;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..(s.len()-1))
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i+2], 16))
		.collect()
}

fn run() -> Result<(), Box<dyn Error>> {
	let artifacts = "../Artifacts/Addition.bin-runtime";
	//println!("{}", Path::new(artifacts).exists());
	let mut file = File::open(artifacts).expect("File not found");
	let mut buffer = String::new();
	file.read_to_string(&mut buffer).expect("Error while reading file");

	let bytes = decode(&buffer)?;

	for b in &bytes {
		println!("0x{:x}", b) 
	}
	println!("{}", buffer);

	loop {
		match vm.next() {
			Some(x) => println!("{:?}", x),
			None => {}
		}
	}
	
	Ok(())
}

fn main() {
	run().unwrap();
}

// VM
struct Vm {
	code: Vec<u8>, // smart contract code
	pc: usize, // current instruction
}

enum Opcode {
	STOP, // 0x00
	ADD, // 0x01
	MUL, // 0x02

	PUSH1(u8), // 0x60
	PUSH2(u8, u8), // 0x61

	PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 
}

impl Vm {
	fn new_from_file(filename: &str) -> Result<Vm, Box<Error>> {
		let mut f = File::open(filename)?;
		let mut buffer = String::new();
		f.read_to_string(&mut buffer)?;
		let code = decode(&buffer)?;
		Ok(Vm { code: code, pc: 0})
	}

	fn next(&mut self) -> Option<Opcode> {
		match self.code[self.pc] {
			0x00 => {
				self.pc += 1;
				Some(Opcode::STOP)
			},
			0x01 => {
				self.pc += 1;
				Some(Opcode::ADD)
			},
			0x02 => {
				self.pc += 1;
				Some(Opcode::MUL)
			},
			0x60 => {
				let value = self.code[self.pc+1];
				self.pc += 2;
				Some(Opcode::PUSH1(value))
			},
			Ox61 => {
				let value0 = self.code[self.pc+1];
				let value1 = self.code[self.pc+2];
				self.pc += 3;
				Some(Opcode::PUSH2(value0, value1))
			}
			_ => { self.pc += 1; None}
		}
	}
}
