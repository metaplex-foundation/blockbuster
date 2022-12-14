#[cfg(test)]
use anchor_lang::{prelude::*, InstructionData};
use blockbuster::{
    instruction::{InstructionBundle, IxPair},
    program_handler::ProgramParser,
    programs::{bubblegum::BubblegumParser, ProgramParseResult},
};
use borsh::ser::BorshSerialize;
use flatbuffers::FlatBufferBuilder;
use helpers::{build_bubblegum_bundle, build_txn_from_fixture, random_list_of, random_pubkey};
pub use mpl_bubblegum::id as program_id;
use mpl_bubblegum::state::{
    leaf_schema::{LeafSchema, Version},
    metaplex_adapter::{Creator, MetadataArgs, TokenProgramVersion},
    BubblegumEventType,
};
use plerkle_serialization::{CompiledInstruction, Pubkey};
use spl_account_compression::{
    events::{
        AccountCompressionEvent, ApplicationDataEvent, ApplicationDataEventV1, ChangeLogEvent,
    },
    state::PathNode,
};
use std::fs;
mod helpers;

#[test]
fn test_setup() {
    let subject = BubblegumParser {};
    assert_eq!(subject.key(), program_id());
    assert!(subject.key_match(&program_id()));
}

#[test]
fn test_mint() {
    let subject = BubblegumParser {};

    let accounts = random_list_of(8, |_i| Pubkey(random_pubkey().to_bytes()));
    let account_indexes: Vec<u8> = accounts.iter().enumerate().map(|(i, _)| i as u8).collect();
    let message = MetadataArgs {
        name: "test".to_string(),
        symbol: "test".to_string(),
        uri: "www.solana.pos".to_owned(),
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: false,
        edition_nonce: None,
        token_standard: None,
        token_program_version: TokenProgramVersion::Original,
        collection: None,
        uses: None,
        creators: vec![Creator {
            address: random_pubkey(),
            verified: false,
            share: 20,
        }],
    };
    let ix = mpl_bubblegum::instruction::MintV1 { message };

    let lse = mpl_bubblegum::state::leaf_schema::LeafSchemaEvent {
        event_type: BubblegumEventType::LeafSchemaEvent,
        version: Version::V1,
        schema: LeafSchema::V1 {
            id: random_pubkey(),
            owner: random_pubkey(),
            delegate: random_pubkey(),
            nonce: 0,
            data_hash: [0; 32],
            creator_hash: [0; 32],
        },
        leaf_hash: [0; 32],
    };

    let cs = ChangeLogEvent::new(
        random_pubkey(),
        vec![PathNode {
            node: [0; 32],
            index: 0,
        }],
        0,
        0,
    );

    let cs_event = AccountCompressionEvent::ChangeLog(cs);

    let lse = mpl_bubblegum::state::leaf_schema::LeafSchemaEvent {
        event_type: BubblegumEventType::LeafSchemaEvent,
        version: Version::V1,
        schema: LeafSchema::V1 {
            id: random_pubkey(),
            owner: random_pubkey(),
            delegate: random_pubkey(),
            nonce: 0,
            data_hash: [0; 32],
            creator_hash: [0; 32],
        },
        leaf_hash: [0; 32],
    };

    let lse_versioned = ApplicationDataEventV1 {
        application_data: lse.try_to_vec().unwrap(),
    };

    let lse_event =
        AccountCompressionEvent::ApplicationData(ApplicationDataEvent::V1(lse_versioned));

    let cs = ChangeLogEvent::new(
        random_pubkey(),
        vec![PathNode {
            node: [0; 32],
            index: 0,
        }],
        0,
        0,
    );

    let cs_event = AccountCompressionEvent::ChangeLog(cs);
    let ix_data = ix.data();
    let mut ix_b = InstructionBundle::default();
    // this is horrifying, we need to re write the flatbuffers sdk
    let mut fbb1 = FlatBufferBuilder::new();
    let mut fbb2 = FlatBufferBuilder::new();
    let mut fbb3 = FlatBufferBuilder::new();
    let mut fbb4 = FlatBufferBuilder::new();

    build_bubblegum_bundle(
        &mut fbb1,
        &mut fbb2,
        &mut fbb3,
        &mut fbb4,
        &accounts,
        &account_indexes,
        &ix_data,
        lse,
        cs_event,
        &mut ix_b,
    );

    let result = subject.handle_instruction(&ix_b);

    if let ProgramParseResult::Bubblegum(b) = result.unwrap().result_type() {
        let matched = match b.instruction {
            mpl_bubblegum::InstructionName::MintV1 => Ok(()),
            _ => Err(()),
        };
        assert!(matched.is_ok());
        assert!(b.payload.is_some());
        assert!(b.leaf_update.is_some());
        assert!(b.tree_update.is_some());
    } else {
        panic!("Unexpected ProgramParseResult variant");
    }
}

#[test]
fn test_basic_success_parsing() {
    let subject = BubblegumParser {};

    let accounts = random_list_of(8, |_i| Pubkey(random_pubkey().to_bytes()));
    let account_indexes: Vec<u8> = accounts.iter().enumerate().map(|(i, _)| i as u8).collect();

    let ix = mpl_bubblegum::instruction::Transfer {
        creator_hash: [0; 32],
        index: 0,
        data_hash: [0; 32],
        nonce: 0,
        root: [0; 32],
    };

    let lse = mpl_bubblegum::state::leaf_schema::LeafSchemaEvent {
        event_type: BubblegumEventType::LeafSchemaEvent,
        version: Version::V1,
        schema: LeafSchema::V1 {
            id: random_pubkey(),
            owner: random_pubkey(),
            delegate: random_pubkey(),
            nonce: 0,
            data_hash: [0; 32],
            creator_hash: [0; 32],
        },
        leaf_hash: [0; 32],
    };

    let lse_versioned = ApplicationDataEventV1 {
        application_data: lse.try_to_vec().unwrap(),
    };

    let lse_event =
        AccountCompressionEvent::ApplicationData(ApplicationDataEvent::V1(lse_versioned));

    let cs = ChangeLogEvent::new(
        random_pubkey(),
        vec![PathNode {
            node: [0; 32],
            index: 0,
        }],
        0,
        0,
    );

    let cs_event = AccountCompressionEvent::ChangeLog(cs);
    let ix_data = ix.data();
    let mut ix_b = InstructionBundle::default();
    // this is horrifying, we need to re write the flatbuffers sdk
    let mut fbb1 = FlatBufferBuilder::new();
    let mut fbb2 = FlatBufferBuilder::new();
    let mut fbb3 = FlatBufferBuilder::new();
    let mut fbb4 = FlatBufferBuilder::new();

    build_bubblegum_bundle(
        &mut fbb1,
        &mut fbb2,
        &mut fbb3,
        &mut fbb4,
        &accounts,
        &account_indexes,
        &ix_data,
        lse,
        cs_event,
        &mut ix_b,
    );
    let result = subject.handle_instruction(&ix_b);

    if let ProgramParseResult::Bubblegum(b) = result.unwrap().result_type() {
        assert!(b.payload.is_none());
        let matched = match b.instruction {
            mpl_bubblegum::InstructionName::Transfer => Ok(()),
            _ => Err(()),
        };
        assert!(matched.is_ok());
        assert!(b.leaf_update.is_some());
        assert!(b.tree_update.is_some());
    } else {
        panic!("Unexpected ProgramParseResult variant");
    }
}
