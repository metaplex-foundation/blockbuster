pub mod bubblegum;
pub mod common;
pub mod token_account;
// pub mod gummyroll; Technichally we dont need to index this as a GP

pub enum ParsedProgram {
    Bubblegum,
    Gummyroll,
    TokenAccount,
}
