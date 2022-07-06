

mod evm;
use evm::vm::Vm;
use evm::opcode::Opcode;
use std::error::Error;
//use std::env;

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
	let filename = "../../Artifacts/Addition.bin-runtime"; //args[2].clone();

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
