use bubblegum::BubblegumInstruction;
use token_metadata::TokenMetadataAccountState;

pub mod bubblegum;
pub mod token_metadata;
// pub mod gummyroll;

pub enum ProgramParseResult<'a> {
    Bubblegum(&'a BubblegumInstruction),
    TokenMetadata(&'a TokenMetadataAccountState),
    Unknown,
}
