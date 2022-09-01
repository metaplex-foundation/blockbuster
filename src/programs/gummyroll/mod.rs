use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{ProgramParser},
};

use mpl_bubblegum::{get_instruction_type};
use borsh::de::BorshDeserialize;
use solana_sdk::{
    pubkey::{
        Pubkey
    },
};
use solana_sdk::pubkeys;
use async_trait::async_trait;


use spl_compression::events::ChangeLogEvent;
use anchor_lang::Discriminator;
use plerkle_serialization::account_info_generated::account_info::AccountInfo;

pubkeys!(
    GummyRollProgramID,
    "GRoLLzvxpxxu2PGNJMMeZPyMxjAUH9pKqxGXV9DGiceU"
);

pub struct Gummyroll {}

pub struct CompressionInstruction {
    tree_update: Option<ChangeLogEvent>,
}

#[async_trait]
impl ProgramParser<CompressionInstruction, ()> for Gummyroll {
    fn key() -> Pubkey {
        GummyRollProgramID()
    }
    fn key_match(key: &Pubkey) -> bool {
        key == &GummyRollProgramID()
    }
    async fn handle_account(_account_info: &AccountInfo) -> Result<(), BlockbusterError> {
        Ok(())
    }
    async fn handle_instruction(bundle: &InstructionBundle) -> Result<CompressionInstruction, BlockbusterError> {
        let InstructionBundle {
            instruction,
            inner_ix,
            ..
        } = bundle;
        let _ix_type = get_instruction_type(instruction.data().unwrap());
        let change_log_event: &[u8] = &ChangeLogEvent::discriminator();
        if let Some(ixs) = inner_ix {
            for ix in ixs {
                if ix.0.0 == wrapper::id().to_bytes() {
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


