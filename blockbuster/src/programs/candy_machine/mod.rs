use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{ProgramParser},
};

use mpl_bubblegum::{get_instruction_type};
use borsh::de::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkeys;

use plerkle_serialization::account_info_generated::account_info::AccountInfo;
use mpl_bubblegum::state::metaplex_adapter::MetadataArgs;

pub use mpl_bubblegum::InstructionName;
pub use mpl_bubblegum::state::leaf_schema::{
    LeafSchemaEvent,
    LeafSchema,
};
use mpl_token_metadata::state::{Key, Edition, MasterEditionV2, Metadata, ReservationListV1, MasterEditionV1, ReservationListV2, EditionMarker, UseAuthorityRecord, CollectionAuthorityRecord};

use mpl_candy_machine::state::CandyMachines;

pubkeys!(
    candy_machine_id,
    "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ"
);

pub enum TokenMetadataAccountData {
    Uninitialized,
    EditionV1(Edition),
    MasterEditionV1(MasterEditionV1),
    MetadataV1(Metadata),
    MasterEditionV2(MasterEditionV2),
    EditionMarker(EditionMarker),
    UseAuthorityRecord(UseAuthorityRecord),
    CollectionAuthorityRecord(CollectionAuthorityRecord),
    

    CandyMachine(CandyMachine)
}

pub struct TokenMetadataAccountState {
    key: Key,
    data: TokenMetadataAccountData,
}

pub struct TokenMetadataParser;

impl ProgramParser for TokenMetadataParser {
    fn key(&self) -> Pubkey {
        token_metadata_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &token_metadata_id()
    }

}
