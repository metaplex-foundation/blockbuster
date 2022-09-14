use crate::program_handler::ParseResult;
use crate::{
    error::BlockbusterError, instruction::InstructionBundle, program_handler::ProgramParser,
};
use crate::{program_handler::NotUsed, programs::ProgramParseResult};
use borsh::de::BorshDeserialize;
use solana_sdk::borsh::try_from_slice_unchecked;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkeys;

use mpl_bubblegum::state::metaplex_adapter::MetadataArgs;
use plerkle_serialization::account_info_generated::account_info::AccountInfo;

pub use mpl_bubblegum::state::leaf_schema::{LeafSchema, LeafSchemaEvent};
pub use mpl_bubblegum::InstructionName;
use mpl_token_metadata::state::{
    CollectionAuthorityRecord, Edition, EditionMarker, Key, MasterEditionV1, MasterEditionV2,
    Metadata, ReservationListV1, ReservationListV2, TokenMetadataAccount, UseAuthorityRecord,
};

pubkeys!(
    token_metadata_id,
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
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
    ReservationListV2(ReservationListV2),
    ReservationListV1(ReservationListV1),
}

pub struct TokenMetadataAccountState {
    key: Key,
    data: TokenMetadataAccountData,
}

impl TokenMetadataAccountState {
    pub fn new(key: Key) -> Self {
        TokenMetadataAccountState { key, data: todo!() }
    }
}

impl ParseResult for TokenMetadataAccountState {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::TokenMetadata(self)
    }
}

pub struct TokenMetadataParser;

impl ProgramParser for TokenMetadataParser {
    fn key(&self) -> Pubkey {
        token_metadata_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &token_metadata_id()
    }

    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        let account_raw_data = account_info.data().unwrap();

        let data = account_raw_data[8..].to_owned();
        let data_buf = &mut data.as_slice();
        let metadata = 
        let mut token_metadata_account_state = TokenMetadataAccountState::new(metadata.key);

        match metadata.key {
            EditionV1 => {
                let data: Edition = try_from_slice_unchecked(data_buf)?;
                token_metadata_account_state.data =
                    TokenMetadataAccountData::EditionV1(data)
            }
            MasterEditionV2 => {
                token_metadata_account_state.data =
                    try_from_slice_unchecked(&metadata.data).unwrap()
            }
        }

        Ok(Box::new(token_metadata_account_state))
    }

    fn handle_instruction(
        &self,
        _bundle: &InstructionBundle,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        Ok(Box::new(NotUsed::new()))
    }
}
