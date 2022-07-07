
pub struct Glaube {
	pub code: Vec<u8>, // smart contract code
	pub pc: usize, // current instruction
	pub at_end: bool,
}

impl Glaube {
	pub fn pray() {
		println!("Unknown opcode");
	}
}