mod balances;
mod system;
mod types;

use crate::types::types::{AccountId, Balance, BlockNumber, Nonce};

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = AccountId;
	type BlockNumber = BlockNumber;
	type Nonce = Nonce;
}

impl balances::Config for Runtime {
	type Balance = Balance;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	println!("Hello, world!");
	let mut run_time = Runtime::new();
	let alice: String = "alice".to_string();
	let bob: String = "bob".to_string();
	let charlie: String = "charlie".to_string();
	run_time.balances.set_balance(&alice, 100);
	run_time.system.inc_block_number();
	assert_eq!(run_time.system.block_number(), 1);

	run_time.system.inc_nonce(&alice);
	let _res = run_time.balances.transfer(&alice, &bob, 30).map_err(|err| eprintln!("{}", err));
	run_time.system.inc_nonce(&alice);

	let _res = run_time
		.balances
		.transfer(&alice, &charlie, 20)
		.map_err(|err| eprintln!("{}", err));
	run_time.system.inc_nonce(&alice);

	println!("{:#?}", run_time);
}
