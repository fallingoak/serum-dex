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
            /// The nonce used to create the Registry's program-derived address,
            /// which owns it's token vault.
            nonce: u8,
        },
        /// Deposits funds into the registry for reward distribution.
        ///
        /// 0. `[signer]`   Owner of the account sending the funds.
        /// 1. `[writable]` Account from which to send the funds.
        /// 2. `[writable]` Program controlled token vault to transfer funds
        ///                 into.
        /// 3. `[]`         Registry instance, holding the nonce to calculate
        ///                 the program-derived-address
        Deposit {
            /// The amount to deposit.
            amount: u64,
        },
        /// CreateEntity initializes the new "node" with the Registry, allowing
        /// addresses to stake wtih it and collect rewards.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Entity account.
        /// 1. `[signed]`   Leader of the node.
        /// 2. `[]`         SPL token program.
        /// 3. `[]`         Rent sysvar.
        CreateEntity { capabilities: u64 },
        /// Stake deposits funds into a registered node entity pool,
        /// initializing the given beneficiary as a staker.
        ///
        /// Accounts:
        ///
        /// 0. `[signed]`   Owner of the account to send funds from.
        /// 1. `[writable]` Token account containing the funds.
        /// 2. `[]`         Registry instance.
        /// 3. `[writable]` Program controlled token vault to transfer
        ///                 funds into.
        /// 4. `[]`         Entity account to stake with.
        /// 5. `[]`         SPL token program.
        Stake {
            /// The amount to stake.
            amount: u64,
            /// The key to associate wtih this deposit. Collecting rewards
            /// will require this key to sign.
            beneficiary: Pubkey,
        },
        /// Collect rewards retrieves rewards earned from node duties.
        ///
        /// Accounts:
        ///
        /// 0. `[signed]`   Beneficiary of the Stake account to collect rewards.
        /// 1. `[writable]` Stake account to collect rewards from.
        /// 2. `[writable]`
        CollectRewards,
    }
}

serum_common::packable!(crate::instruction::RegistryInstruction);
