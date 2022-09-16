use bubblegum::BubblegumInstruction;
use candy_machine::CandyMachineAccountData;
use token_account::TokenProgramAccount;

// pub mod token_metadata;
pub mod bubblegum;
pub mod candy_machine;
pub mod token_account;
// pub mod gummyroll;

pub enum ProgramParseResult<'a> {
    Bubblegum(&'a BubblegumInstruction),
    TokenProgramAccount(&'a TokenProgramAccount),
    CandyMachine(&'a CandyMachineAccountData),
    Unknown,
}
