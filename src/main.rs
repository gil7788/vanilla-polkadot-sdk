mod balances;
mod proof_of_existence;
mod support;
mod system;
mod types;

use crate::{
	support::{Dispatch, DispatchResult, Extrinsic},
	types::{BalancesPallet, Runtime, RuntimeCall, SystemConfig, SystemPallet, types::Block},
};

impl Runtime {
	fn new() -> Self {
		Self { 
			system: SystemPallet::new(),
			balances: BalancesPallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new()
		 }
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
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
			RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            },
		}
		Ok(())
	}
}

fn main() {
    // Create a new instance of the Runtime.
    // It will instantiate with it all the modules it uses.
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // Initialize the system with some initial balance.
    runtime.balances.set_balance(&alice, 100);

    // Here are the extrinsics in our block.
    // You can add or remove these based on the modules and calls you have set up.
    let block_1 =  crate::support::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
            },
        ],
    };

    let block_2 = crate::support::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "Hello, world!".to_string(),
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "Hello, world!".to_string(),
                }),
            },
        ],
    };

    let block_3 = crate::support::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
                    claim: "Hello, world!".to_string(),
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "Hello, world!".to_string(),
                }),
            },
        ],
    };

    // Execute the extrinsics which make up our blocks.
    // If there are any errors, our system panics, since we should not execute invalid blocks.
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");

    // Simply print the debug format of our runtime state.
    println!("{runtime:#?}");
}