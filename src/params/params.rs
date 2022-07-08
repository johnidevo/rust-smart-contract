//use self::uint::U256;
use primitive_types::U256;

pub struct InputParameters {
	data: Vec<u8>,
}

impl InputParameters {

	pub fn new(data: Vec<u8>) -> InputParameters {
		InputParameters { data }
	}

	pub fn get(&self, index: usize) -> U256 {
		self.data[index..index+32].into()
	}

	pub fn size(&self) -> U256 {
		U256::from(self.data.len())
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	#[test]
	fn test_parameters_ok() {
		let data = (0..32).collect();

		let params = InputParameters::new(data);
		let size = params.size();

		assert_eq!(32, size.as_u32());
		let bigint = params.get(0);

		assert_eq!(31, bigint.byte(0));
		assert_eq!(0, bigint.byte(31));
	}
}
