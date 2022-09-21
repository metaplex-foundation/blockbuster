use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{ParseResult, ProgramParser},
};

use crate::{program_handler::NotUsed, programs::ProgramParseResult};
use anchor_lang::Discriminator;
use borsh::de::BorshDeserialize;
use mpl_bubblegum::{get_instruction_type, state::metaplex_adapter::MetadataArgs};
pub use mpl_bubblegum::{
    id as program_id,
    state::leaf_schema::{LeafSchema, LeafSchemaEvent},
    InstructionName,
};
use plerkle_serialization::AccountInfo;
use solana_sdk::pubkey::Pubkey;
pub use spl_account_compression::events::ChangeLogEvent;
use spl_noop;

#[derive(Eq, PartialEq)]
pub enum Payload {
    Unknown,
    MintV1 { args: MetadataArgs },
    Decompress { args: MetadataArgs },
    CancelRedeem { root: [u8; 32] },
    VerifyCreator { creator: Pubkey },
    UnverifyCreator { creator: Pubkey },
}

pub struct BubblegumInstruction {
    pub instruction: InstructionName,
    pub tree_update: Option<ChangeLogEvent>,
    pub leaf_update: Option<LeafSchemaEvent>,
    pub payload: Option<Payload>,
}

impl BubblegumInstruction {
    pub fn new(ix: InstructionName) -> Self {
        BubblegumInstruction {
            instruction: ix,
            tree_update: None,
            leaf_update: None,
            payload: None,
        }
    }
}

impl ParseResult for BubblegumInstruction {
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::Bubblegum(self)
    }
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

pub struct BubblegumParser;

impl ProgramParser for BubblegumParser {
    fn key(&self) -> Pubkey {
        program_id()
    }

    fn key_match(&self, key: &Pubkey) -> bool {
        key == &program_id()
    }
    fn handle_account(
        &self,
        _account_info: &AccountInfo,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        Ok(Box::new(NotUsed::new()))
    }

    fn handle_instruction(
        &self,
        bundle: &InstructionBundle,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        let InstructionBundle {
            instruction,
            inner_ix,
            keys,
            ..
        } = bundle;

        let ix_type = get_instruction_type(instruction.data().unwrap());
        let mut b_inst = BubblegumInstruction::new(ix_type);

        let leaf_event: &[u8] = &LeafSchemaEvent::discriminator();
        let change_log_event: &[u8] = &ChangeLogEvent::discriminator();

        if let Some(ixs) = inner_ix {
            for ix in ixs {
                if ix.0.0 == spl_noop::id().to_bytes() {
                    let cix = ix.1;
                    if let Some(data) = cix.data() {
                        let disc = &data[0..8];
                        if disc == leaf_event {
                            println!("{:?}", ix.0.0);
                            let event = LeafSchemaEvent::try_from_slice(data)?;
                            b_inst.leaf_update = Some(event)
                        }
                        if disc == change_log_event {
                            let event = ChangeLogEvent::try_from_slice(data)?;
                            b_inst.tree_update = Some(event)
                        }
                    }
                }
            }
        }

        if let Some(data) = instruction.data() {
            match b_inst.instruction {
                InstructionName::MintV1 => {
                    let args: MetadataArgs = MetadataArgs::try_from_slice(data)?;
                    b_inst.payload = Some(Payload::MintV1 { args });
                }
                InstructionName::DecompressV1 => {
                    let args: MetadataArgs = MetadataArgs::try_from_slice(data)?;
                    b_inst.payload = Some(Payload::Decompress { args });
                }
                InstructionName::CancelRedeem => {
                    let slice: [u8; 32] = data
                        .try_into()
                        .map_err(|_e| BlockbusterError::InstructionParsingError)?;
                    b_inst.payload = Some(Payload::CancelRedeem { root: slice });
                }
                InstructionName::VerifyCreator => {
                    b_inst.payload = Some(Payload::VerifyCreator {
                        creator: Pubkey::new_from_array(keys.get(3).unwrap().0),
                    });
                }
                InstructionName::UnverifyCreator => {
                    b_inst.payload = Some(Payload::UnverifyCreator {
                        creator: Pubkey::new_from_array(keys.get(3).unwrap().0),
                    });
                }
                _ => {}
            };
        }
        Ok(Box::new(b_inst))
    }
}
