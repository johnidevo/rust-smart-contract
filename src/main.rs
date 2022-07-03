use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len()-1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

fn main() {
	let mut file = File::open("../Artifacts/Addition.bin-runtime").expect("File not found");
	let mut buffer = String::new();
	file.read_to_string(&mut buffer).expect("Error while reading file");
	println!("{}", buffer);
	let bytes = decode(&buffer)?;

	for b in &bytes {
			println!("0x{:x}", b) 
	}
	println!("{}", buffer);
}