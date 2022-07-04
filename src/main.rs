use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::error::Error;
use std::path::Path;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..(s.len()-1))
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i+2], 16))
		.collect()
}

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


	Ok(())
}

fn main() {
	run().unwrap();
}
