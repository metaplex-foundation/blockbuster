use crate::program_handler::ProgramMatcher;
use crate::{
    error::BlockbusterError, instruction::InstructionBundle, program_handler::ProgramParser,
};

pubkeys!(
    TokenProgramID,
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

pub struct TokenAccountParser;

impl ProgramMatcher for TokenAccountParser {
    fn key() -> Pubkey {
        TokenProgramID()
    }
    fn key_match(key: &Pubkey) -> bool {
        key == &TokenProgramID()
    }
}

impl ProgramParser<BubblegumInstruction, ()> for TokenAccountParser {
    fn handle_account(account_info: &AccountInfo) -> Result<(), BlockbusterError> {
        if account_info.data.len() != TokenAccount::LEN {
            return Ok(());
        }
        let token_account = TokenAccount::unpack_unchecked(&account_info.data)
            .context("Failed to deserialize token account data!")?;
        Ok(())
    }

    fn handle_instruction(_bundle: &InstructionBundle) -> Result<(), BlockbusterError> {
        Ok(())
    }
}
