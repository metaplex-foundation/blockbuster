use bubblegum::BubblegumInstruction;

// pub mod token_metadata;
pub mod bubblegum;
// pub mod gummyroll;

pub enum ProgramParseResult<'a> {
    Bubblegum(&'a BubblegumInstruction),
    Unknown
}

