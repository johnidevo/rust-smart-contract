
mod evm;
use evm::vm::Vm;
use evm::opcode::Opcode;
use std::error::Error;
//use std::env;
use std::io;

fn debug(vm: &mut Vm) {
	loop {
		if vm.at_end {
			break;
		}

println!("Correctly loaded loop");
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

/*
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
*/
/*
fn interpret(vm: &mut Vm) {
	while !vm.at_end {
		vm.interpret();
	}
	vm.print_stack();
}
*/
fn run() -> Result<(), Box<dyn Error>> {

	//let args: Vec<String> = env::args().collect();
	let function = "debug"; //args[1].clone();
	let filename = "./Artifacts/Addition.bin-runtime"; //args[2].clone();

	println!("In file {}", filename);

	let mut vm = Vm::new_from_file(&filename)?;
	println!("Correctly loaded VM");

	match &*function {
		"debug" => debug(&mut vm),
		//"run" => interpret(&mut vm),
		_ => panic!("Expect either 'debug' or 'run' for first parameter")
	}


	Ok(())
}

fn main() {
    run().unwrap();
}
