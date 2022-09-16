use bubblegum::BubblegumInstruction;
use token_metadata::TokenMetadataAccountState;
use token_account::TokenProgramAccount;

pub mod bubblegum;
pub mod token_metadata;
pub mod token_account;
// pub mod gummyroll;

pub enum ProgramParseResult<'a> {
    Bubblegum(&'a BubblegumInstruction),
    TokenMetadata(&'a TokenMetadataAccountState),
    TokenProgramAccount(&'a TokenProgramAccount),
    Unknown,
}
