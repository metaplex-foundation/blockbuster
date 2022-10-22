use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{ParseResult, ProgramParser},
};

use crate::{program_handler::NotUsed, programs::ProgramParseResult};
use borsh::de::BorshDeserialize;
use mpl_bubblegum::{
    get_instruction_type,
    state::{metaplex_adapter::MetadataArgs, BubblegumEventType},
};
pub use mpl_bubblegum::{
    id as program_id,
    state::leaf_schema::{LeafSchema, LeafSchemaEvent},
    InstructionName,
};
use plerkle_serialization::AccountInfo;
use solana_sdk::pubkey::Pubkey;
pub use spl_account_compression::events::{
    AccountCompressionEvent::{self, ApplicationData, ChangeLog},
    ApplicationDataEvent, ChangeLogEvent, ChangeLogEventV1,
};
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
//TODO add more of the parsing here to minimize program transformer code
pub struct BubblegumInstruction {
    pub instruction: InstructionName,
    pub tree_update: Option<ChangeLogEventV1>,
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
    fn handles_account_updates(&self) -> bool {
        false
    }

    fn handles_instructions(&self) -> bool {
        true
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
        let data = instruction.data();
        if data.is_none() {
            return Err(BlockbusterError::InstructionParsingError);
        }
        let ix_type = get_instruction_type(data.unwrap());
        let mut b_inst = BubblegumInstruction::new(ix_type);
        if let Some(ixs) = inner_ix {
            for ix in ixs {
                if ix.0 .0 == spl_noop::id().to_bytes() {
                    let cix = ix.1;
                    if let Some(data) = cix.data() {
                        if !data.is_empty() {
                            match AccountCompressionEvent::try_from_slice(data)? {
                                ChangeLog(changelog_event) => {
                                    let ChangeLogEvent::V1(changelog_event) = changelog_event;
                                    b_inst.tree_update = Some(changelog_event);
                                }
                                ApplicationData(app_data) => {
                                    let ApplicationDataEvent::V1(app_data) = app_data;
                                    let app_data = app_data.application_data;

                                    let event_type_byte = if !app_data.is_empty() {
                                        &app_data[0..1]
                                    } else {
                                        return Err(BlockbusterError::DeserializationError);
                                    };

                                    match BubblegumEventType::try_from_slice(event_type_byte)? {
                                        BubblegumEventType::Uninitialized => {
                                            return Err(
                                                BlockbusterError::MissingBubblegumEventData,
                                            );
                                        }
                                        BubblegumEventType::LeafSchemaEvent => {
                                            b_inst.leaf_update =
                                                Some(LeafSchemaEvent::try_from_slice(&app_data)?);
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        return Err(BlockbusterError::InstructionParsingError);
                    }
                }
            }
        }

        if let Some(ix_data) = data.map(|d| &d[8..]) {
            if !ix_data.is_empty() {
                match b_inst.instruction {
                    InstructionName::MintV1 => {
                        let args: MetadataArgs = MetadataArgs::try_from_slice(ix_data)?;
                        b_inst.payload = Some(Payload::MintV1 { args });
                    }
                    InstructionName::DecompressV1 => {
                        let args: MetadataArgs = MetadataArgs::try_from_slice(ix_data)?;
                        b_inst.payload = Some(Payload::Decompress { args });
                    }
                    InstructionName::CancelRedeem => {
                        let slice: [u8; 32] = ix_data
                            .try_into()
                            .map_err(|_e| BlockbusterError::InstructionParsingError)?;
                        b_inst.payload = Some(Payload::CancelRedeem { root: slice });
                    }
                    InstructionName::VerifyCreator => {
                       let creator = keys.get(3).ok_or(BlockbusterError::InstructionParsingError)?.0;

                        b_inst.payload = Some(Payload::VerifyCreator {
                            creator: Pubkey::new_from_array(creator),
                        });
                    }
                    InstructionName::UnverifyCreator => {
                        let creator = keys.get(3).ok_or(BlockbusterError::InstructionParsingError)?.0;
                        b_inst.payload = Some(Payload::UnverifyCreator {
                            creator: Pubkey::new_from_array(creator),
                        });
                    }
                    _ => {}
                };
            }
        } else {
            return Err(BlockbusterError::InstructionParsingError);
        }
        Ok(Box::new(b_inst))
    }
}
