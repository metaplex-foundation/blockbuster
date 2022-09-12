use crate::program_handler::ProgramMatcher;
use crate::{
    error::BlockbusterError, instruction::InstructionBundle, program_handler::ProgramParser,
};

pubkeys!(
    TokenProgramID,
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

pub struct TokenAccountParser;

impl ProgramMatcher for TokenAccountParser {
    fn key() -> Pubkey {
        TokenProgramID()
    }
    fn key_match(key: &Pubkey) -> bool {
        key == &TokenProgramID()
    }
}

impl ProgramParser<BubblegumInstruction, ()> for TokenAccountParser {
    fn handle_account(account_info: &AccountInfo) -> Result<(), BlockbusterError> {
        if account_info.data.len() != TokenAccount::LEN {
            return Ok(());
        }
        let token_account = TokenAccount::unpack_unchecked(&account_info.data)
            .context("Failed to deserialize token account data!")?;

        let pubkey = account_info.key.to_string();

        // if amount != 1 {
        //     return Ok(());
        // }

        let owner = token_account.owner.to_vec();
        let mint_address = token_account.mint.to_string();
        let incoming_slot: i64 = slot.try_into()?;

        let metadata = if let Some(account_data) = account_update.data() {
            let data = account_data[8..].to_owned();
            let data_buf = &mut data.as_slice();
            Metadata::deserialize(data_buf)?
        } else {
            // todo: give relevant error
            return Err(IngesterError::CompressedAssetEventMalformed);
        };

        let chain_data = ChainDataV1 {
            name: metadata.data.name,
            symbol: metadata.data.symbol,
            edition_nonce: metadata.edition_nonce,
            primary_sale_happened: metadata.primary_sale_happened,
            token_standard: metadata
                .token_standard
                .and_then(|ts| TokenStandard::from_u8(ts as u8)),
            uses: metadata.uses.map(|u| Uses {
                use_method: UseMethod::from_u8(u.use_method as u8).unwrap(),
                remaining: u.remaining,
                total: u.total,
            }),
        };

        let model = asset::ActiveModel {
            id: Set(metadata.mint.to_bytes().to_vec()),
            owner: Set(owner),
            owner_type: Set(OwnerType::Single),
            delegate: Set(None),
            frozen: Set(false),
            supply: Set(1),
            supply_mint: Set(None),
            compressed: Set(true),
            compressible: Set(false),
            tree_id: Set(None),
            specification_version: Set(1),
            nonce: Set(nonce as i64),
            leaf: Set(None),
            royalty_target_type: Set(RoyaltyTargetType::Creators),
            royalty_target: Set(None),
            royalty_amount: Set(metadata.data.seller_fee_basis_points as i32), //basis points
            chain_data_id: Set(Some(data.id)),
            seq: Set(0),
            ..Default::default()
        };

        Ok(())
    }

    fn handle_instruction(_bundle: &InstructionBundle) -> Result<(), BlockbusterError> {
        Ok(())
    }
}
