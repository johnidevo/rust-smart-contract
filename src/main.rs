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

/*
impl Opcode {
    fn describe(&self) {
        match self {
        Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
        Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
        Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
        Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
        Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
        _ => println!("Unknown opcode")
 
        }
    }
}


*/
#[derive(Debug)]
enum Opcode {
    STOP(usize), // 0x00
    ADD(usize), // 0x01
    MUL(usize), // 0x02

    PUSH1(usize, u8), // 0x60
    PUSH2(usize, u8, u8), // 0x61
    PUSH32(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 

    EOF,
}


// update run function
fn run() -> Result<(), Box<dyn Error>> {

	let artifacts = "./Artifacts/Addition.bin-runtime"; //command path gap, cargo run
    
    println!("In file {}", artifacts);

    let mut vm = Vm::new_from_file(&artifacts)?;

    loop {
        match vm.next() {
            Some(Opcode::EOF) => break,
            Some(x) => println!("{:?}", x),
            None => {}
        }
    }
    Ok(())
}

/*
fn run() -> Result<(), Box<dyn Error>> {
	let artifacts = "./Artifacts/Addition.bin-runtime"; //command path gap, cargo run
	println!("{}", Path::new(artifacts).exists());
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
*/
fn main() {
	run().unwrap();
}

struct Vm {
    code: Vec<u8>, // smart contract code
    pc: usize, // current instruction
}

impl Vm {
    fn new_from_file(filename: &str) -> Result<Vm, Box<dyn Error>> {
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


			
        Ok(Vm { code: code, pc: 0})
    }

    fn next(&mut self) -> Option<Opcode> {
        match self.code[self.pc] {
            _ => { self.pc += 1; None}
        }
    }
}

