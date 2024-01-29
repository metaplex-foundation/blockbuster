use bubblegum::BubblegumInstruction;
use token_account::TokenProgramAccount;
use token_metadata::TokenMetadataAccountState;

pub mod bubblegum;
pub mod token_account;
pub mod token_metadata;

// Note: `ProgramParseResult` used to contain the following variants that have been deprecated and
// removed from blockbuster since the `version-1.16` tag of blockbuster:
// CandyGuard(&'a CandyGuardAccountData),
// CandyMachine(&'a CandyMachineAccountData),
// CandyMachineCore(&'a CandyMachineCoreAccountData),
//
// The reason Candy Machine V3 parsing was removed was because Candy Guard (`mpl-candy-guard`) and
// Candy Machine V3 (`mpl-candy-machine-core`) were dependent upon a specific Solana version (1.16)
// at the time, there was no Candy Machine parsing in DAS (`digital-asset-rpc-infrastructure`), and
// we wanted to use the Rust clients for Token Metadata and Bubblegum so that going forward we could
// more easily update blockbuster to new Solana versions going forward.
//
// Candy Machine V2 (`mpl-candy-machine`) parsing did not depend on the `mpl-candy-machine` crate
// as its types were copied, but we removed V2 parsing at the same time as V3 parsing as it was not
// being used.
pub enum ProgramParseResult<'a> {
    Bubblegum(&'a BubblegumInstruction),
    TokenMetadata(&'a TokenMetadataAccountState),
    TokenProgramAccount(&'a TokenProgramAccount),
    Unknown,
}
