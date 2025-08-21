use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::collections::BTreeMap;
use crate::balances;
use crate::proof_of_existence;

pub mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, super::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
	pub type Content = String;
}

// Main
#[derive(Debug)]
pub struct Runtime {
	pub system: SystemPallet<Self>,
	pub balances: BalancesPallet<Self>,
	pub proof_of_existence: proof_of_existence::Pallet<Self>,
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	// BalancesTransfer { to: types::AccountId, amount: types::Balance },
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
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
