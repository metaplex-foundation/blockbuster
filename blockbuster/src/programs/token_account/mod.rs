use crate::program_handler::ParseResult;
use crate::{
    error::BlockbusterError, instruction::InstructionBundle, program_handler::ProgramParser,
};
use crate::{program_handler::NotUsed, programs::ProgramParseResult};
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

impl ParseResult for TokenProgramAccount {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::TokenProgramAccount(self)
    }
}

impl ProgramParser for TokenAccountParser {
    fn key(&self) -> Pubkey {
        TokenProgramID()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &TokenProgramID()
    }

    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
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

        Ok(Box::new(account_type))
    }

    fn handle_instruction(
        &self,
        _bundle: &InstructionBundle,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        Ok(Box::new(NotUsed::new()))
    }
}
