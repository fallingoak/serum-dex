use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

pub use entity::Entity;

/// Registry defines the account representing an instance of the program.
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Registry {
    pub mint: Pubkey,
    pub initialized: bool,
    pub nonce: u8,
}

pub mod entity {
    /// Entity is the account representing a single "node".
    #[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Entity {
        pub initialized: bool,
    }
}

/// Staker represents an individuals staking deposit with an entity in the
/// registry.
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Stake {
    pub initialized: bool,
    pub beneficiary: Pubkey,
    pub entity_id: Pubkey,
    pub amount: u64,
}

serum_common::packable!(Registry);
