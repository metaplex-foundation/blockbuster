use bubblegum::BubblegumInstruction;
use token_account::TokenProgramAccount;

// pub mod token_metadata;
pub mod bubblegum;
pub mod token_account;
// pub mod gummyroll;

pub enum ProgramParseResult<'a> {
    Bubblegum(&'a BubblegumInstruction),
    TokenProgramAccount(&'a TokenProgramAccount),
    Unknown,
}
