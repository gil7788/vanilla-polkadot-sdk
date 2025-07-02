use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::collections::BTreeMap;

pub mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, super::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

// Main
#[derive(Debug)]
pub struct Runtime {
	pub system: SystemPallet<Self>,
	pub balances: BalancesPallet<Self>,
}

pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

// Balances
#[derive(Debug)]
pub struct BalancesPallet<T: BalancesConfig> {
	pub balances: BTreeMap<T::AccountId, T::Balance>,
}

pub trait BalancesConfig: SystemConfig {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

//  System
#[derive(Debug)]
pub struct SystemPallet<T: SystemConfig> {
	pub block_number: T::BlockNumber,
	pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

pub trait SystemConfig {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + CheckedAdd + Copy;
	type Nonce: Zero + One + Copy;
}
