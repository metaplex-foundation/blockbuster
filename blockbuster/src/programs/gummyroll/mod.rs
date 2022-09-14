use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{ProgramParser, ProgramMatcher},
};

use borsh::de::BorshDeserialize;
use solana_sdk::{
    pubkey::{
        Pubkey
    },
};
use solana_sdk::pubkeys;
use spl_account_compression::events::ChangeLogEvent;
use anchor_lang::Discriminator;
use plerkle_serialization::account_info_generated::account_info::AccountInfo;

pubkeys!(
    gummy_roll_program_id,
    "GRoLLzvxpxxu2PGNJMMeZPyMxjAUH9pKqxGXV9DGiceU"
);

pub struct GummyrollParser {}

pub struct CompressionInstruction {
    pub tree_update: Option<ChangeLogEvent>,
}


impl ProgramParser for GummyrollParser {
    fn key() -> Pubkey {
        gummy_roll_program_id()
    }
    fn key_match(key: &Pubkey) -> bool {
        key == &gummy_roll_program_id()
    }
    fn handle_account(_account_info: &AccountInfo) -> Result<(), BlockbusterError> {
        Ok(())
    }
    fn handle_instruction(bundle: &InstructionBundle) -> Result<CompressionInstruction, BlockbusterError> {
        let InstructionBundle {
            instruction,
            inner_ix,
            ..
        } = bundle;
        let change_log_event: &[u8] = &ChangeLogEvent::discriminator();
        if let Some(ixs) = inner_ix {
            for ix in ixs {
                if ix.0.0 == spl_noop::id().to_bytes() {
                    let cix = ix.1;
                    if let Some(data) = cix.data() {
                        let disc = &data[0..8];
                        if disc == change_log_event {
                            let event = ChangeLogEvent::try_from_slice(data)?;
                            return Ok(CompressionInstruction {
                                tree_update: Some(event)
                            });
                        }
                    }
                }
            }
        }
        Err(BlockbusterError::InstructionParsingError)
    }
}


