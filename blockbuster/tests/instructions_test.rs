#[cfg(test)]
mod helpers;
use anchor_lang::AnchorDeserialize;
use blockbuster::{
    instruction::{order_instructions, InstructionBundle, IxPair},
    program_handler::ProgramParser,
    programs::{
        bubblegum::{BubblegumParser, Payload},
        token_metadata::{LeafSchema, LeafSchemaEvent},
        ProgramParseResult,
    },
};
use flatbuffers::FlatBufferBuilder;
use helpers::*;
use plerkle_serialization::{root_as_transaction_info, Pubkey as FBPubkey, TransactionInfo};
use rand::prelude::IteratorRandom;
use solana_sdk::pubkey::Pubkey;
use spl_account_compression::events::{
    AccountCompressionEvent::{self, ApplicationData},
    ApplicationDataEvent, ApplicationDataEventV1, ChangeLogEvent, ChangeLogEventV1,
};
use std::collections::{HashSet, VecDeque};
use std::env;
#[test]
fn test_filter() {
    let mut rng = rand::thread_rng();
    let fbb = FlatBufferBuilder::new();
    let fbb = build_random_transaction(fbb);
    let data = fbb.finished_data();
    let txn = root_as_transaction_info(data).expect("TODO: panic message");
    let programs = get_programs(txn);
    let chosen_progs = programs.iter().choose_multiple(&mut rng, 3);
    let mut hs = HashSet::new();
    chosen_progs.iter().fold(&mut hs, |hs, p| {
        hs.insert(p.as_ref());
        hs
    });
    let _len = hs.len();
    let hsb = hs.clone();
    let res = order_instructions(hs, &txn);
    for (ib, _inner) in res.iter() {
        let public_key_matches = hsb.contains(&ib.0 .0.as_ref());
        assert!(public_key_matches);
    }

    let res = order_instructions(HashSet::new(), &txn);
    assert_eq!(res.len(), 0);
}

fn prepare_fixture<'a>(
    fbb: &'a mut FlatBufferBuilder<'a>,
    fixture: &'a str,
) -> TransactionInfo<'a> {
    println!("{:?}", env::current_dir());
    let name = fixture.to_string();
    let fbb = build_txn_from_fixture(name.clone(), fbb).unwrap();
    root_as_transaction_info(fbb.finished_data()).expect("Fail deser")
}

#[test]
fn helium_nested() {
    let mut fbb = FlatBufferBuilder::new();
    let txn = prepare_fixture(&mut fbb, "helium_nested");
    let mut prog = HashSet::new();
    let id = mpl_bubblegum::id();
    let slot = txn.slot();
    prog.insert(id.as_ref());
    let res = order_instructions(prog, &txn);
    let accounts = txn.account_keys().unwrap();
    let mut keys: Vec<FBPubkey> = Vec::with_capacity(accounts.len());
    for k in accounts.into_iter() {
        keys.push(*k);
    }

    let _ix = 0;

    let contains = res
        .iter()
        .any(|(ib, _inner)| ib.0 .0.as_ref() == mpl_bubblegum::id().as_ref());
    assert!(contains, "Must containe bgum at hoisted root");
    let subject = BubblegumParser {};
    for (outer_ix, inner_ix) in res.into_iter() {
        let (program, instruction) = outer_ix;
        let ix_accounts = instruction.accounts().unwrap().iter().collect::<Vec<_>>();
        let ix_account_len = ix_accounts.len();
        let _max = ix_accounts.iter().max().copied().unwrap_or(0) as usize;
        let ix_accounts =
            ix_accounts
                .iter()
                .fold(Vec::with_capacity(ix_account_len), |mut acc, a| {
                    if let Some(key) = keys.get(*a as usize) {
                        acc.push(*key);
                    }
                    //else case here is handled on 272
                    acc
                });
        let bundle = InstructionBundle {
            txn_id: "",
            program,
            instruction: Some(instruction),
            inner_ix,
            keys: ix_accounts.as_slice(),
            slot,
        };
        let result = subject.handle_instruction(&bundle).unwrap();
        let res_type = result.result_type();
        let parse_result = match res_type {
            ProgramParseResult::Bubblegum(parse_result) => parse_result,
            _ => panic!("Wrong type"),
        };

        if let (Some(le), Some(cl), Some(Payload::MintV1 { args: _ })) = (
            &parse_result.leaf_update,
            &parse_result.tree_update,
            &parse_result.payload,
        ) {
        } else {
            panic!("Failed to parse instruction");
        }
    }
}

#[test]
fn test_double_mint() {
    let mut fbb = FlatBufferBuilder::new();
    let txn = prepare_fixture(&mut fbb, "double_bubblegum_mint");
    let mut programs = HashSet::new();
    let subject = BubblegumParser {}.key();
    programs.insert(subject.as_ref());
    let ix = order_instructions(programs, &txn);
    let contains = ix
        .iter()
        .filter(|(ib, _inner)| ib.0 .0.as_ref() == mpl_bubblegum::id().as_ref());
    assert_eq!(contains.count(), 2);
    assert_eq!(ix.len(), 2);
    for i in ix {
        let (program, instruction) = i.0;
        let inner = i.1.unwrap();
        for ii in &inner {
            println!("{:?}", Pubkey::new(&ii.0 .0.as_ref()));
        }
        let ace =
            AccountCompressionEvent::try_from_slice(inner[1].1.data().unwrap().bytes()).unwrap();
        if let AccountCompressionEvent::ApplicationData(ApplicationDataEvent::V1(
            ApplicationDataEventV1 {
                application_data, ..
            },
        )) = ace
        {
            let lse = LeafSchemaEvent::try_from_slice(&application_data).unwrap();
            let LeafSchema::V1 {
                id,
                owner,
                delegate,
                nonce,
                data_hash,
                creator_hash,
            } = lse.schema;

            println!("Nonce {}", nonce);
        } else {
            panic!("Failed to parse instruction");
        }
        let cle =
            AccountCompressionEvent::try_from_slice(inner[3].1.data().unwrap().bytes()).unwrap();
        if let AccountCompressionEvent::ChangeLog(ChangeLogEvent::V1(ChangeLogEventV1 {
            id, ..
        })) = cle
        {
            println!("ID {:?}", id);
        } else {
            panic!("Failed to parse instruction");
        }

        assert_eq!(inner.len(), 4);
    }
}

#[test]
fn test_double_tree() {
    let mut fbb = FlatBufferBuilder::new();
    let txn = prepare_fixture(&mut fbb, "helium_mint_double_tree");
    let mut programs = HashSet::new();
    let subject = BubblegumParser {}.key();
    programs.insert(subject.as_ref());
    let ix = order_instructions(programs, &txn);
    let contains = ix
        .iter()
        .filter(|(ib, _inner)| ib.0 .0.as_ref() == mpl_bubblegum::id().as_ref());
    
    contains.for_each(|bix| {
        if let Some(inner) = &bix.1 {
            for ii in inner {
                println!("pp {:?}", Pubkey::new(&ii.0 .0.as_ref()));
            }
            println!("------");
            let cl = AccountCompressionEvent::try_from_slice(inner[8].1.data().unwrap().bytes())
                .unwrap();
            if let AccountCompressionEvent::ChangeLog(ChangeLogEvent::V1(ChangeLogEventV1 {
                id,
                ..
            })) = cl
            {
                println!("Merkle Tree {:?}", id);
            }
            let cl = AccountCompressionEvent::try_from_slice(inner[16].1.data().unwrap().bytes())
                .unwrap();
            if let AccountCompressionEvent::ChangeLog(ChangeLogEvent::V1(ChangeLogEventV1 {
                id,
                ..
            })) = cl
            {
                println!("Merkle Tree {:?}", id);
            }
        }
    });
    
    
}
