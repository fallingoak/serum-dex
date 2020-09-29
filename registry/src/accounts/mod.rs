use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Registry {
		pub mint: Pubkey,
		pub initialized: bool,
		pub nonce: u8,
}

serum_common::packable!(Registry);
