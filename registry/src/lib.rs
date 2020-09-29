//! serum-safe defines the interface for the serum safe program.

#![cfg_attr(feature = "strict", deny(warnings))]

use serum_common::pack::*;
use solana_client_gen::prelude::*;

pub mod accounts;
pub mod error;

#[cfg_attr(feature = "client", solana_client_gen)]
pub mod instruction {
    use super::*;
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum RegistryInstruction {
				/// Initializes the registry instance for use.
				///
				/// Accounts:
				///
				/// 0. `[writable]` Registry to initialize.
				/// 1. `[]`         Mint of the SPL token to pay rewards with.
				/// 2. `[]`         Rent sysvar.
				Initialize {
						/// The nonce used to create the Registry's program-derived address.
						nonce: u8,
				},
        /// Accounts:
        ///
        /// 0. `[signed]` Leader of the node.
        /// 1. `[]`
        CreateEntity {
						capabilities: u64,
				},
				/// Accounts:
				///
				/// 0. `[signed]`
				/// 1. `[writable]` The program controlled token vault to transfer
				///                 funds into.
				///
        Stake {
						entity_id: Pubkey,
				},
				/// Accounts:
				///
				/// 0. `[signed]`
				CollectRewards,
    }
}

serum_common::packable!(crate::instruction::RegistryInstruction);
