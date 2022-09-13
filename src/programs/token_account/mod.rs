use crate::program_handler::ProgramMatcher;
use crate::{
    error::BlockbusterError, instruction::InstructionBundle, program_handler::ProgramParser,
};
use plerkle_serialization::account_info_generated::account_info::AccountInfo;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkeys;
use spl_token::state::{Account as TokenAccount, Mint};

pubkeys!(
    TokenProgramID,
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

pub struct TokenAccountParser;

pub enum TokenProgramAccount {
    Mint(Mint),
    TokenAccount(TokenAccount),
}

impl ProgramMatcher for TokenAccountParser {
    fn key() -> Pubkey {
        TokenProgramID()
    }
    fn key_match(key: &Pubkey) -> bool {
        key == &TokenProgramID()
    }
}

impl ProgramParser<(), TokenProgramAccount> for TokenAccountParser {
    fn handle_account(account_info: &AccountInfo) -> Result<TokenProgramAccount, BlockbusterError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info
        } else {
            return Err(BlockbusterError::DeserializationError);
        };

        let account_type = match account_data.len() {
            165 => {
                let token_account = TokenAccount::unpack(&account_data).unwrap();

                TokenProgramAccount::TokenAccount(token_account)
            }
            82 => {
                let mint = Mint::unpack(&account_data).unwrap();

                TokenProgramAccount::Mint(mint)
            }
            _ => {
                return Err(BlockbusterError::InvalidDataLength);
            }
        };

        Ok(account_type)
    }

    fn handle_instruction(_bundle: &InstructionBundle) -> Result<(), BlockbusterError> {
        Ok(())
    }
}
