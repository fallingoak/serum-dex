//! Program entrypoint.

#![cfg_attr(feature = "strict", deny(warnings))]

use serum_common::pack::Pack;
use serum_registry::error::{RegistryError, RegistryErrorCode};
use serum_registry::instruction::RegistryInstruction;
use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;

solana_sdk::entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    info!("process-instruction");

    let instruction: RegistryInstruction = RegistryInstruction::unpack(instruction_data)
        .map_err(|_| RegistryError::ErrorCode(RegistryErrorCode::WrongSerialization))?;
    info!("STAKING!");

    let result: ProgramResult = match instruction {
        RegistryInstruction::Initialize { nonce, } => {
            info!("Initialize INSTRUCTION!");
            Ok(())
        }
        RegistryInstruction::CreateEntity { capabilities } => {
            info!("REGISTRY INSTRUCTION!");
            Ok(())
        }
        RegistryInstruction::Stake { entity_id } => {
            info!("DELEGATE INSTRUCTION!");
            Ok(())
        }
				RegistryInstruction::CollectRewards {} => {
						info!("Collect rewards");
						Ok(())
				}
    };

    result?;

    info!("process-instruction success");

    Ok(())
}
