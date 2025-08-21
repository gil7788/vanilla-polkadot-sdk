use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::SystemConfig {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	pub claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}

	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("Claim already exists");
		}
		self.claims.insert(claim, caller.clone());
		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("Claim doesn't exist");
		if owner.unwrap() != &caller {
			return Err("Caller is not owner of claim");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

pub enum Call<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::CreateClaim { claim } => self.create_claim(caller, claim)?,
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim)?,
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = String;
	}

	impl crate::SystemConfig for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!".to_string()), None);
		assert_eq!(poe.create_claim("alice".to_string(), "Hello, world!".to_string()), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!".to_string()), Some(&"alice".to_string()));
		assert_eq!(
			poe.create_claim("bob".to_string(), "Hello, world!".to_string()),
			Err("Claim already exists")
		);
		assert_eq!(poe.revoke_claim("alice".to_string(), "Hello, world!".to_string()), Ok(()));
		assert_eq!(poe.create_claim("bob".to_string(), "Hello, world!".to_string()), Ok(()));
	}
}
