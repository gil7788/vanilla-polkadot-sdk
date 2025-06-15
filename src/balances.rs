use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&from);
		let to_balance = self.balance(&to);

		let new_from_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Funds overflow")?;

		self.set_balance(from, new_from_balance);
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	struct TestConfig;
	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_funds() {
		let mut balances = super::Pallet::<TestConfig>::new();

		balances.set_balance(&"alice".to_string(), 100);
		let result = balances.transfer(&"alice".to_string(), &"bob".to_string(), 50);
		assert_eq!(result.unwrap(), ());
		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);
	}

	#[test]
	fn fail_to_transfer_non_existent_funds() {
		let mut balances = super::Pallet::<TestConfig>::new();

		balances.set_balance(&"alice".to_string(), 100);

		let result = balances.transfer(&"alice".to_string(), &"bob".to_string(), 101);

		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), "Not enough funds.");
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}
}
