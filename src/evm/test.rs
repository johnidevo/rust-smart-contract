// evm/vm_test.rs
//use super::vm::Vm;
pub struct Test {
	pub code: Vec<u8>, // smart contract code
	pub pc: usize, // current instruction
	pub at_end: bool,
}
impl Test {

	pub fn addition() {
		println!("Unknown opcode");
	}

}

/*

warning: `read_file` (bin "read_file") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 3.00s
     Running `target/debug/read_file`
In file ./Artifacts/Addition.bin-runtime
Correctly loaded VM
0x0     PUSH1   Place 1-byte item on the stack 0xf
0x2     PUSH1   Place 1-byte item on the stack 0x1
0x4     ADD     Addition operation
0x5     STOP    Halts execution
*/