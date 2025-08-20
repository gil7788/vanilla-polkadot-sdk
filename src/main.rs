mod balances;
mod support;
mod system;
mod types;

use crate::{
	support::{Dispatch, DispatchResult, Extrinsic, Header},
	types::{types::Block, BalancesPallet, Runtime, RuntimeCall, SystemConfig, SystemPallet},
};

impl Runtime {
	fn new() -> Self {
		Self { system: SystemPallet::new(), balances: BalancesPallet::new() }
	}

	fn execute_block(&mut self, block: Block) -> DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("Block number mismatch");
		}

		for (i, Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _result = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as SystemConfig>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(&caller, &to, amount)?;
			},
		}
		Ok(())
	}
}

fn main() {
	println!("Hello, world!");
	
	let mut run_time = Runtime::new();
	let alice: String = "alice".to_string();
	let bob: String = "bob".to_string();
	let charlie: String = "charlie".to_string();

	run_time.balances.set_balance(&alice, 100);
	// run_time.system.inc_block_number();
	// assert_eq!(run_time.system.block_number(), 1);

	// run_time.system.inc_nonce(&alice);
	// let _res = run_time.balances.transfer(&alice, &bob, 30).map_err(|err| eprintln!("{}", err));
	// run_time.system.inc_nonce(&alice);

	// let _res = run_time
	// 	.balances
	// 	.transfer(&alice, &charlie, 20)
	// 	.map_err(|err| eprintln!("{}", err));
	// run_time.system.inc_nonce(&alice);

	let block =  Block {
		header: Header { block_number: 1 },
		extrinsics: {
			vec![
				Extrinsic {
					caller: alice.clone(),
					call: RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 69 }
				},
				Extrinsic {
					caller: alice.clone(),
					call: RuntimeCall::BalancesTransfer { to: charlie, amount: 12 }
				},
				Extrinsic {
					caller: alice.clone(),
					call: RuntimeCall::BalancesTransfer { to: bob, amount: 11 }
				}
			]
		}
	};

	run_time.execute_block(block).expect("invalid block");

	println!("{:#?}", run_time);
}
