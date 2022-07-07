
mod evm;
use evm::vm::Vm;
#[warn(unused_imports)]
use evm::opcode::Opcode;

use evm::memory::Memory as mem;

//use evm::glaube::Glaube;
//use evm::test::Test;

use std::error::Error;
//use std::env;
use std::io;

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
