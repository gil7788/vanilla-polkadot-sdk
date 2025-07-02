use crate::types::{SystemConfig, SystemPallet};
use num::traits::{CheckedAdd, One, Zero};
use std::collections::BTreeMap;

impl<T: SystemConfig> SystemPallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number = self
			.block_number
			.checked_add(&T::BlockNumber::one())
			.unwrap_or(T::BlockNumber::one());
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::SystemConfig for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::SystemPallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get(&"bob".to_string()), None);
	}
}
