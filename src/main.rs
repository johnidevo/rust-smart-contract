
pub mod evm;
pub mod params;
use evm::vm::Vm;
use evm::opcode::Opcode;
use evm::memory::Memory;
use params::params::InputParameters;

use std::error::Error;
//use std::env;
use std::io;


fn create_vm(binary: Vec<u8>) -> Vm {
	//Vm { code: binary, pc: 0, stack: Vec::new(), at_end: false}
	let data = (0..32).collect();
	Vm { 
		code: binary, 
		pc: 0, 
		stack: Vec::new(), 
		mem: Memory::new(), 
		input_data: InputParameters::new(data), 
		at_end: false
	}
}

fn vm_test() {
	// 1 + 5
	let binary = vec![0x60, 0x0f, 0x60, 0x01, 0x01, 0x00];
	let mut vm = create_vm(binary); //moved

	// execute three instructions.
	// push 0x0f
	vm.interpret();
	vm.print_debug();
	// push 0x01
	vm.interpret();
	vm.print_debug();
	// add
	vm.interpret();
	vm.print_debug();
	// halt
	vm.interpret();
	vm.print_debug();

	// Now make sure the stack size is 1 and contains 16.
	assert_eq!(1, vm.stack.len());
	assert_eq!(16, vm.stack[0].as_u32()); // this is panicking in case of overflow.

	vm.print_debug();
}

fn interpret(vm: &mut Vm) {
	
let mut last = 0;
for x in 1..vm.code.len() {
	
		//println!("x:{}", x);
	
    if last > vm.code.len() {
        break;
    }
	
		vm.interpret();
		//vm.print_debug();
	
    last = x;
}
	
	println!("last:{}", last);
	println!("pc:{}", vm.pc);
	println!("code.len:{}\n", vm.code.len());
	vm.print_debug();
/*
	loop {

		if vm.pc >= vm.code.len() {
			break;
		}
		vm.interpret();
		
		vm.print_debug();
	}
	vm.print_debug();
*/
	/*
		if vm.at_end {
			break;
		}

				
    while !vm.at_end {
        vm.interpret();
    }
    vm.print_stack();
	*/

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
	let function = "run"; //args[1].clone();
	let filename = "./Artifacts/04/Example.bin-runtime"; //args[2].clone();

	println!("In file {}", filename);
	//let data = (0..32).collect();
	//let params = InputParameters::new(data);
	let params = InputParameters::new(vec![0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);

	
	let mut vm = Vm::new_from_file(&filename, params)?;
	println!("Correctly loaded VM");

	match &*function {
		"debugger" => debugger(&mut vm),
		"debug" => debug(&mut vm),
		"run" => interpret(&mut vm),
		"test" => vm_test(),
		_ => panic!("Expect either 'debug' or 'run' for first parameter")
	}

	Ok(())
}

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
}

/*
fn main() {
	run().unwrap();
}
*/
/* ################# */

/*
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct User {
    id: i64,
    USR_Email: String,
    USR_Password: String,
    USR_Enabled: i32,
    USR_MAC_Address: String
}

#[post("/", format = "json", data = "<user_input>")]
fn helloPost(user_input: Json<User>) -> String {
    format!("print test {:?}", user_input)
}

fn main() {
    rocket::ignite().mount("/hello", routes![helloPost]).launch();
}

*/


/*
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!   kkdd"
}


#[launch]
fn rocket() -> _ {
    //rocket::build().mount("/", routes![index])
	  rocket::ignite()
        .manage(CodeRepo { contracts: std::sync::Mutex::new(HashMap::new()) })
        .mount("/", routes![transact]).launch();

}
*/

/*
fn main() {
	rocket::ignite()
		.manage(CodeRepo { contracts: std::sync::Mutex::new(HashMap::new()) })
		.mount("/", routes![transact, deploy]).launch();
}

#[post("/transact", format = "json", data = "<message>")]
fn transact(message: Json<TransactionInput>, state: State<CodeRepo>) -> JsonValue {

	let mut code: Vec<u8> = Vec::new();
	{
		let contracts = state.contracts.lock().unwrap();
		match contracts.get(&message.0.to) {
		}
	}
	let input_str = message.0.data;

	
	println!("In file {}", filename);
	//let data = (0..32).collect();
	//let params = InputParameters::new(data);
	let params = InputParameters::new(vec![0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);


	let mut vm = Vm::new_from_file(&filename, params)?;
	println!("Correctly loaded VM");
	
	
	
	

	// No error handling here :D
	let v = vm::decode(&input_str).expect("Input data should be hexadecimal");
	let mut vm = Vm::new(code, InputParameters::new(v));


    while !vm.at_end {
        vm.interpret();
    }


    match vm.status {
        vm::VmStatus::DONE => {
            match vm.stack.pop() {
                //let returned = v.low_u64();
                Ok(v) => json!({"result": ""}), //returned})
                Err => json!({"error": "Tried to return by no value on top of stack"})
            }
        },
        vm::VmStatus::REVERT => json!({"error": "error while running smart contract"}),
        _ => panic!("ABORRTTTTT"),
    }

}

#[derive(Serialize, Deserialize)]
struct DeployInput {
    binary: String,
}

*/



/* ################# */

/*
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

*/

/*
0xf
0x0
0x33
6080604052348015600f57600080fd5b506004361060285760003560e01c8063ed14b6a514602d575b600080fd5b604a60408051808201909152338152600060209091018190528
055565b00fea2646970667358221220b19a69fc931c8a9c097f73c9e9e4985e5175903c9970b56ae2de40184fff5d3264736f6c634300080f0033
Correctly loaded VM
c
[96, 128, 96, 64, 82, 52, 128, 21, 96, 15, 87, 96, 0, 128, 253, 91, 80, 96, 4, 54, 16, 96, 40, 87, 96, 0, 53, 96, 224, 28, 128, 99, 237, 20, 18
2, 165, 20, 96, 45, 87, 91, 96, 0, 128, 253, 91, 96, 74, 96, 64, 128, 81, 128, 130, 1, 144, 145, 82, 51, 129, 82, 96, 0, 96, 32, 144, 145, 1, 1
29, 144, 82, 128, 85, 86, 91, 0, 254, 162, 100, 105, 112, 102, 115, 88, 34, 18, 32, 177, 154, 105, 252, 147, 28, 138, 156, 9, 127, 115, 201, 23
3, 228, 152, 94, 81, 117, 144, 60, 153, 112, 181, 106, 226, 222, 64, 24, 79, 255, 93, 50, 100, 115, 111, 108, 99, 67, 0, 8, 15, 0, 51]
0x0     PUSH1   Place 1-byte item on stack 0x80
c
[96, 128, 96, 64, 82, 52, 128, 21, 96, 15, 87, 96, 0, 128, 253, 91, 80, 96, 4, 54, 16, 96, 40, 87, 96, 0, 53, 96, 224, 28, 128, 99, 237, 20, 18
2, 165, 20, 96, 45, 87, 91, 96, 0, 128, 253, 91, 96, 74, 96, 64, 128, 81, 128, 130, 1, 144, 145, 82, 51, 129, 82, 96, 0, 96, 32, 144, 145, 1, 1
29, 144, 82, 128, 85, 86, 91, 0, 254, 162, 100, 105, 112, 102, 115, 88, 34, 18, 32, 177, 154, 105, 252, 147, 28, 138, 156, 9, 127, 115, 201, 23
3, 228, 152, 94, 81, 117, 144, 60, 153, 112, 181, 106, 226, 222, 64, 24, 79, 255, 93, 50, 100, 115, 111, 108, 99, 67, 0, 8, 15, 0, 51]
0x2     PUSH1   Place 1-byte item on stack 0x40
c
[96, 128, 96, 64, 82, 52, 128, 21, 96, 15, 87, 96, 0, 128, 253, 91, 80, 96, 4, 54, 16, 96, 40, 87, 96, 0, 53, 96, 224, 28, 128, 99, 237, 20, 18
2, 165, 20, 96, 45, 87, 91, 96, 0, 128, 253, 91, 96, 74, 96, 64, 128, 81, 128, 130, 1, 144, 145, 82, 51, 129, 82, 96, 0, 96, 32, 144, 145, 1, 1
29, 144, 82, 128, 85, 86, 91, 0, 254, 162, 100, 105, 112, 102, 115, 88, 34, 18, 32, 177, 154, 105, 252, 147, 28, 138, 156, 9, 127, 115, 201, 23
3, 228, 152, 94, 81, 117, 144, 60, 153, 112, 181, 106, 226, 222, 64, 24, 79, 255, 93, 50, 100, 115, 111, 108, 99, 67, 0, 8, 15, 0, 51]
0x4     MSTORE  Save word to memory
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 64', src/evm/memory.rs:38:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
root@goorm:/workspace/CORDS/snooze(master)# sh .assets/git.sh

*/