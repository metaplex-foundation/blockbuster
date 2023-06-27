use crate::{
    error::BlockbusterError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use borsh::BorshDeserialize;
use solana_sdk::{borsh::try_from_slice_unchecked, pubkey::Pubkey, pubkeys};

use plerkle_serialization::AccountInfo;

pub use mpl_bubblegum::{
    state::leaf_schema::{LeafSchema, LeafSchemaEvent},
    InstructionName,
};
use mpl_token_metadata::{
    state::{
        CollectionAuthorityRecord, Edition, EditionMarker, Key, MasterEditionV1, MasterEditionV2,
        Metadata, ReservationListV1, ReservationListV2, UseAuthorityRecord,
    },
    utils::meta_deser_unchecked,
};

pubkeys!(
    token_metadata_id,
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

#[allow(clippy::large_enum_variant)]
pub enum TokenMetadataAccountData {
    EditionV1(Edition),
    MasterEditionV1(MasterEditionV1),
    MetadataV1(Metadata),
    MasterEditionV2(MasterEditionV2),
    EditionMarker(EditionMarker),
    UseAuthorityRecord(UseAuthorityRecord),
    CollectionAuthorityRecord(CollectionAuthorityRecord),
    ReservationListV2(ReservationListV2),
    ReservationListV1(ReservationListV1),
    EmptyAccount,
}

pub struct TokenMetadataAccountState {
    pub key: Key,
    pub data: TokenMetadataAccountData,
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

    fn handles_account_updates(&self) -> bool {
        true
    }

    fn handles_instructions(&self) -> bool {
        false
    }

    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info.iter().collect::<Vec<_>>()
        } else {
            return Ok(Box::new(TokenMetadataAccountState {
                key: Key::Uninitialized,
                data: TokenMetadataAccountData::EmptyAccount,
            }));
        };
        if account_data.is_empty() {
            return Ok(Box::new(TokenMetadataAccountState {
                key: Key::Uninitialized,
                data: TokenMetadataAccountData::EmptyAccount,
            }));
        }
        let key = Key::try_from_slice(&account_data[0..1])?;
        let token_metadata_account_state = match key {
            Key::EditionV1 => {
                let account: Edition = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::EditionV1(account),
                }
            }
            Key::MasterEditionV1 => {
                let account: MasterEditionV2 = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::MasterEditionV2(account),
                }
            }
            Key::MasterEditionV2 => {
                let account: MasterEditionV1 = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::MasterEditionV1(account),
                }
            }
            Key::UseAuthorityRecord => {
                let account: UseAuthorityRecord = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::UseAuthorityRecord(account),
                }
            }
            Key::EditionMarker => {
                let account: EditionMarker = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::EditionMarker(account),
                }
            }
            Key::CollectionAuthorityRecord => {
                let account: CollectionAuthorityRecord = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::CollectionAuthorityRecord(account),
                }
            }
            Key::MetadataV1 => {
                let account: Metadata = meta_deser_unchecked(&mut account_data.as_slice())?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::MetadataV1(account),
                }
            }
            Key::ReservationListV1 => {
                let account: ReservationListV1 = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::ReservationListV1(account),
                }
            }
            Key::ReservationListV2 => {
                let account: ReservationListV2 = try_from_slice_unchecked(&account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::ReservationListV2(account),
                }
            }
            Key::Uninitialized => {
                return Err(BlockbusterError::UninitializedAccount);
            }
            _ => {
                return Err(BlockbusterError::AccountTypeNotImplemented);
            }
        };

        Ok(Box::new(token_metadata_account_state))
    }
}
