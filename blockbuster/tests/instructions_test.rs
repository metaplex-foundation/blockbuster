mod helpers;

use blockbuster::instruction::order_instructions;
use flatbuffers::FlatBufferBuilder;
use helpers::*;
use plerkle_serialization::root_as_transaction_info;
use rand::prelude::IteratorRandom;

use std::collections::HashSet;

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
