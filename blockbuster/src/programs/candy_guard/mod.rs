use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{NotUsed, ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use mpl_candy_guard::{
    guards::MintCounter,
    state::{CandyGuard, CandyGuardData, DATA_OFFSET},
};
use plerkle_serialization::AccountInfo;
use solana_sdk::{borsh::try_from_slice_unchecked, pubkey::Pubkey, pubkeys};
use std::convert::TryInto;

pubkeys!(
    candy_guard_id,
    "Guard1JwRhJkVH6XZhzoYxeBVQe872VH6QggF4BWmS9g"
);

// Anchor account discriminators.
const CANDY_GUARD_DISCRIMINATOR: [u8; 8] = [44, 207, 199, 184, 112, 103, 34, 181];
const MINT_COUNTER_DISCRIMINATOR: [u8; 8] = [29, 59, 15, 69, 46, 22, 227, 173];

pub enum CandyGuardAccountData {
    CandyGuard(CandyGuard, Box<CandyGuardData>),
    MintCounter(MintCounter),
}

impl ParseResult for CandyGuardAccountData {
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::CandyGuard(self)
    }
}

pub struct CandyGuardParser;

impl ProgramParser for CandyGuardParser {
    fn key(&self) -> Pubkey {
        candy_guard_id()
    }

    fn key_match(&self, key: &Pubkey) -> bool {
        key == &candy_guard_id()
    }
    fn handles_account_updates(&self) -> bool {
        true
    }

    fn handles_instructions(&self) -> bool {
        false
    }
    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<dyn ParseResult>, BlockbusterError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info
        } else {
            return Err(BlockbusterError::DeserializationError);
        };

        let discriminator: [u8; 8] = account_data[0..8].try_into().unwrap();

        let account_type = match discriminator {
            CANDY_GUARD_DISCRIMINATOR => {
                let candy_guard = try_from_slice_unchecked(&account_data[8..])?;
                let candy_guard_data =
                    CandyGuardData::load(&account_data[DATA_OFFSET..]).map_err(|_| {
                        BlockbusterError::CustomDeserializationError(
                            "Candy Guard Data Deserialization Error".to_string(),
                        )
                    })?;
                CandyGuardAccountData::CandyGuard(candy_guard, candy_guard_data)
            }
            MINT_COUNTER_DISCRIMINATOR => {
                let mint_counter = try_from_slice_unchecked(&account_data[8..])?;
                CandyGuardAccountData::MintCounter(mint_counter)
            }
            _ => return Err(BlockbusterError::UnknownAccountDiscriminator),
        };

        Ok(Box::new(account_type))
    }

    fn handle_instruction(
        &self,
        _bundle: &InstructionBundle,
    ) -> Result<Box<dyn ParseResult>, BlockbusterError> {
        Ok(Box::new(NotUsed::new()))
    }
}
