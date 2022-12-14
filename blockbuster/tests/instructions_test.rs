#[cfg(test)]
mod helpers;
use blockbuster::{
    instruction::{order_instructions, InstructionBundle, IxPair},
    program_handler::ProgramParser,
    programs::{
        bubblegum::{BubblegumParser, Payload},
        ProgramParseResult,
    },
};
use flatbuffers::FlatBufferBuilder;
use helpers::*;
use plerkle_serialization::{root_as_transaction_info, Pubkey as FBPubkey};
use rand::prelude::IteratorRandom;
use std::collections::{HashSet, VecDeque};
use std::{env, fs};

#[test]
fn test_filter() {
    let mut rng = rand::thread_rng();
    let fbb = FlatBufferBuilder::new();
    let fbb = build_random_transaction(fbb);
    let data = fbb.finished_data();
    let txn = root_as_transaction_info(data).expect("TODO: panic message");
    println!("\t\t accounts {:?}", txn.account_keys());
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
        println!("\t\t matching {:?}", ib.0);
        let public_key_matches = hsb.contains(&ib.0 .0.as_ref());
        assert!(public_key_matches);
    }

    let res = order_instructions(HashSet::new(), &txn);
    assert_eq!(res.len(), 0);
}

#[test]
fn test_fixtures() {
    println!("{:?}", env::current_dir());
    let files = fs::read_dir("tests/fixtures").unwrap();
    let fitxtures = files
        .filter_map(|d| d.ok())
        .filter(|d| d.path().extension().unwrap() == "json")
        .map(|d| d.path().file_stem().unwrap().to_owned())
        .collect::<Vec<_>>();

    for fixture in fitxtures {
        let fbb = FlatBufferBuilder::new();
        let name = fixture.into_string().unwrap();
        let fbb = build_txn_from_fixture(name.clone(), fbb).unwrap();

        let data = fbb.finished_data();
        let txn = root_as_transaction_info(data).expect("Fail deser");
        let mut prog = HashSet::new();
        let id = mpl_bubblegum::id();
        prog.insert(id.as_ref());
        let res = order_instructions(prog, &txn);
        let accounts = txn.account_keys().unwrap();
        let mut va: Vec<FBPubkey> = Vec::with_capacity(accounts.len());
        for k in accounts.into_iter() {
            va.push(*k);
        }
        if name == "helium_nested" {
            helium_nested(res, txn.slot(), &va);
        }
    }
}

fn helium_nested<'a>(
    res: VecDeque<(IxPair<'a>, Option<Vec<IxPair<'a>>>)>,
    slot: u64,
    keys: &[FBPubkey],
) {
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
            println!("{:?} {:?}", le, cl.id);
        } else {
            panic!("Failed to parse instruction");
        }
    }
}
