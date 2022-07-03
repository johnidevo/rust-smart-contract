use std::fs::File;
use std::io::prelude::*;

fn main() {
	let mut file = File::open("../Artifacts/Addition.bin-runtime").expect("File not found");
	let mut data = String::new();
	file.read_to_string(&mut data).expect("Error while reading file");
	println!("{}", data);

}