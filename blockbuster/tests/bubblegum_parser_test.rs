extern crate core;

use crate::helpers::{build_instruction, random_list_of, random_pubkey};
use anchor_lang::{prelude::*, InstructionData};
use blockbuster::{
    instruction::{InstructionBundle, IxPair},
    program_handler::ProgramParser,
    programs::{bubblegum::BubblegumParser, ProgramParseResult},
};
use flatbuffers::FlatBufferBuilder;
pub use mpl_bubblegum::id as program_id;
use mpl_bubblegum::state::{
    leaf_schema::{LeafSchema, Version},
    BubblegumEventType,
};
use plerkle_serialization::Pubkey;
use spl_account_compression::{
    events::{
        AccountCompressionEvent, ApplicationDataEvent, ApplicationDataEventV1, ChangeLogEvent,
    },
    state::PathNode,
};
use borsh::ser::BorshSerialize;

mod helpers;

#[test]
fn test_setup() {
    let subject = BubblegumParser {};
    assert_eq!(subject.key(), program_id());
    assert!(subject.key_match(&program_id()));
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

    let mut fbb = FlatBufferBuilder::new(); // I really REALLLY hate this
    let outer_ix = build_instruction(&mut fbb, &ix.data(), &account_indexes).unwrap();
    let mut fbb = FlatBufferBuilder::new();
    let lse = lse_event.try_to_vec().unwrap();
    let noop_bgum = spl_noop::instruction(lse).data;
    let noop_bgum_ix = (
        Pubkey(spl_noop::id().to_bytes()),
        build_instruction(&mut fbb, &noop_bgum, &account_indexes).unwrap(),
    );
    let mut fbb = FlatBufferBuilder::new();
    // The Compression Instruction here doesnt matter only the noop but we add it here to ensure we are validating that one Account compression event is happening after Bubblegum
    let gummy_roll_ix: IxPair = (
        Pubkey(spl_account_compression::id().to_bytes()),
        build_instruction(&mut fbb, &[0; 0], &account_indexes).unwrap(),
    );
    let mut fbb = FlatBufferBuilder::new();
    let cs = cs_event.try_to_vec().unwrap();
    let noop_compression = spl_noop::instruction(cs).data;
    let noop_compression_ix = (
        Pubkey(spl_noop::id().to_bytes()),
        build_instruction(&mut fbb, &noop_compression, &account_indexes).unwrap(),
    );

    let inner_ix = vec![noop_bgum_ix, gummy_roll_ix, noop_compression_ix];

    let bundle = InstructionBundle {
        txn_id: "",
        program: Pubkey(program_id().to_bytes()),
        instruction: outer_ix,
        inner_ix: Some(inner_ix),
        keys: accounts.as_slice(),
        slot: 0,
    };
    let result = subject.handle_instruction(&bundle);

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
