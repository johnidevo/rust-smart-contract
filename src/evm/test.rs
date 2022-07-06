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