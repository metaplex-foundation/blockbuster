use solana_sdk::pubkey::Pubkey;
use plerkle_serialization::account_info_generated::account_info::AccountInfo;
use async_trait::async_trait;
use crate::{
    instruction::InstructionBundle,
    error::BlockbusterError,
};

pub trait ProgramParser<IO, AO>: Sync + Send {
    fn handle_account(
        account_info: &AccountInfo,
    ) -> Result<AO, BlockbusterError>;
    fn handle_instruction(bundle: &InstructionBundle) -> Result<IO, BlockbusterError>;
}

#[async_trait]
pub trait ProgramMatcher: Sync + Send {
    fn key() -> Pubkey;
    fn key_match(key: &Pubkey) -> bool;
}


