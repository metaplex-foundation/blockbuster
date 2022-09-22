extern crate core;

use crate::helpers::{build_random_account_update, random_list, random_pubkey};
use blockbuster::{
    error::BlockbusterError,
    program_handler::ProgramParser,
    programs::{
        candy_machine::{
            candy_machine_id,
            state::{
                CandyMachine as CandyMachineAccountType,
                CandyMachineData as CandyMachineDataAccountType,
                CollectionPDA as CollectionPDAAccountType, Creator, EndSettingType, EndSettings,
                FreezePDA as FreezePDAAccountType, GatekeeperConfig, HiddenSettings,
                WhitelistMintMode, WhitelistMintSettings,
            },
            CandyMachineAccountData::{CandyMachine, CollectionPDA, FreezePDA},
            CandyMachineParser, CANDY_MACHINE_DISCRIMINATOR, COLLECTION_PDA_DISCRIMINATOR,
            FREEZE_PDA_DISCRIMINATOR,
        },
        ProgramParseResult,
    },
};

use borsh::BorshSerialize;
use flatbuffers::FlatBufferBuilder;

mod helpers;

#[test]
fn test_setup() {
    let subject = CandyMachineParser {};
    assert_eq!(subject.key(), candy_machine_id());
    assert!(subject.key_match(&candy_machine_id()));
}

fn get_test_candy_machine() -> CandyMachineAccountType {
    // Create CandyMachine test data.
    let end_settings = EndSettings {
        end_setting_type: EndSettingType::Amount,
        number: 5000,
    };

    let creators = vec![
        Creator {
            address: random_pubkey(),
            verified: true,
            share: 33,
        },
        Creator {
            address: random_pubkey(),
            verified: false,
            share: 33,
        },
        Creator {
            address: random_pubkey(),
            verified: false,
            share: 33,
        },
        Creator {
            address: random_pubkey(),
            verified: true,
            share: 1,
        },
    ];

    let hidden_settings = HiddenSettings {
        name: String::from("name"),
        uri: String::from("uri"),
        hash: random_list(32, u8::MAX).try_into().unwrap(),
    };

    let whitelist_mint_settings = WhitelistMintSettings {
        mode: WhitelistMintMode::BurnEveryTime,
        mint: random_pubkey(),
        presale: true,
        discount_price: Some(12345),
    };

    let gatekeeper_config = GatekeeperConfig {
        gatekeeper_network: random_pubkey(),
        expire_on_use: true,
    };

    let candy_machine_data = CandyMachineDataAccountType {
        uuid: String::from("uri"),
        price: 991177,
        symbol: String::from("ABC"),
        seller_fee_basis_points: 44,
        max_supply: 100000,
        is_mutable: true,
        retain_authority: false,
        go_live_date: Some(1663833216),
        end_settings: Some(end_settings),
        creators,
        hidden_settings: Some(hidden_settings),
        whitelist_mint_settings: Some(whitelist_mint_settings),
        items_available: 55,
        gatekeeper: Some(gatekeeper_config),
    };

    CandyMachineAccountType {
        authority: random_pubkey(),
        wallet: random_pubkey(),
        token_mint: Some(random_pubkey()),
        items_redeemed: 33,
        data: candy_machine_data,
    }
}

#[test]
fn test_basic_success_parsing_candy_machine_account() {
    // Get CandyMachine test data.
    let test_candy_machine = get_test_candy_machine();

    // Borsh serialize the CandyMachine test data.
    let mut data = CANDY_MACHINE_DISCRIMINATOR.to_vec();
    test_candy_machine
        .serialize(&mut data)
        .expect("Could not serialize candy machine data");

    // Flatbuffer serialize the data.
    let mut fbb = FlatBufferBuilder::new();
    let account_info =
        build_random_account_update(&mut fbb, &data).expect("Could not build account update");

    // Use `CandyMachineParser` to parse the account update.
    let subject = CandyMachineParser {};
    let result = subject.handle_account(&account_info);
    assert!(result.is_ok());

    // Check `ProgramParseResult` and make sure the data is parsed and matches the test data.
    if let ProgramParseResult::CandyMachine(candy_machine_account_data) =
        result.unwrap().result_type()
    {
        match candy_machine_account_data {
            CandyMachine(parsed_candy_machine) => {
                assert_eq!(*parsed_candy_machine, Box::new(test_candy_machine));
            }
            _ => panic!("Unexpected CandyMachineAccountData variant"),
        }
    } else {
        panic!("Unexpected ProgramParseResult variant");
    }
}

#[test]
fn test_unknown_discriminator_fails() {
    // Borsh serialize the CandyMachine discriminator.
    let mut data = CANDY_MACHINE_DISCRIMINATOR.to_vec();

    // Corrupt the discriminator.
    data[0] = 0;

    // Flatbuffer serialize the data.
    let mut fbb = FlatBufferBuilder::new();
    let account_info =
        build_random_account_update(&mut fbb, &data).expect("Could not build account update");

    // Use `CandyMachineParser` to parse the account update.
    let subject = CandyMachineParser {};
    let result = subject.handle_account(&account_info);

    // Validate expected error.
    assert!(result.is_err());
    if let Err(err) = result {
        match err {
            BlockbusterError::UnknownAccountDiscriminator => (),
            _ => panic!("Unexpected error: {}", err),
        }
    }
}

#[test]
fn test_wrong_size_candy_machine_account_fails() {
    // Borsh serialize the CandyMachine discriminator.
    let mut data = CANDY_MACHINE_DISCRIMINATOR.to_vec();
    // Add some random data.
    data.append(&mut random_list(32, u8::MAX));

    // Flatbuffer serialize the data.
    let mut fbb = FlatBufferBuilder::new();
    let account_info =
        build_random_account_update(&mut fbb, &data).expect("Could not build account update");

    // Use `CandyMachineParser` to parse the account update.
    let subject = CandyMachineParser {};
    let result = subject.handle_account(&account_info);

    // Validate expected error.
    assert!(result.is_err());
    if let Err(err) = result {
        match err {
            BlockbusterError::IOError(_) => (),
            _ => panic!("Unexpected error: {}", err),
        }
    }
}

#[test]
fn test_basic_success_parsing_collection_pda_account() {
    // Create CollectionPDA test data.
    let test_collection_pda = CollectionPDAAccountType {
        mint: random_pubkey(),
        candy_machine: random_pubkey(),
    };

    // Borsh serialize the CandyMachine test data.
    let mut data = COLLECTION_PDA_DISCRIMINATOR.to_vec();
    test_collection_pda
        .serialize(&mut data)
        .expect("Could not serialize CollectionPDA data");

    // Flatbuffer serialize the data.
    let mut fbb = FlatBufferBuilder::new();
    let account_info =
        build_random_account_update(&mut fbb, &data).expect("Could not build account update");

    // Use `CandyMachineParser` to parse the account update.
    let subject = CandyMachineParser {};
    let result = subject.handle_account(&account_info);
    assert!(result.is_ok());

    // Check `ProgramParseResult` and make sure the data is parsed and matches the test data.
    if let ProgramParseResult::CandyMachine(candy_machine_account_data) =
        result.unwrap().result_type()
    {
        match candy_machine_account_data {
            CollectionPDA(parsed_collection_pda) => {
                assert_eq!(*parsed_collection_pda, test_collection_pda);
            }
            _ => panic!("Unexpected CandyMachineAccountData variant"),
        }
    } else {
        panic!("Unexpected ProgramParseResult variant");
    }
}

#[test]
fn test_basic_success_parsing_freeze_pda_account() {
    // Create FreezePDA test data.
    let test_freeze_pda = FreezePDAAccountType {
        candy_machine: random_pubkey(),
        allow_thaw: true,
        frozen_count: 3,
        mint_start: Some(1663833216),
        freeze_time: 300,
        freeze_fee: 1000000,
    };

    // Borsh serialize the CandyMachine test data.
    let mut data = FREEZE_PDA_DISCRIMINATOR.to_vec();
    test_freeze_pda
        .serialize(&mut data)
        .expect("Could not serialize FreezePDA data");

    // Flatbuffer serialize the data.
    let mut fbb = FlatBufferBuilder::new();
    let account_info =
        build_random_account_update(&mut fbb, &data).expect("Could not build account update");

    // Use `CandyMachineParser` to parse the account update.
    let subject = CandyMachineParser {};
    let result = subject.handle_account(&account_info);
    assert!(result.is_ok());

    // Check `ProgramParseResult` and make sure the data is parsed and matches the test data.
    if let ProgramParseResult::CandyMachine(candy_machine_account_data) =
        result.unwrap().result_type()
    {
        match candy_machine_account_data {
            FreezePDA(parsed_freeze_pda) => {
                assert_eq!(*parsed_freeze_pda, test_freeze_pda);
            }
            _ => panic!("Unexpected CandyMachineAccountData variant"),
        }
    } else {
        panic!("Unexpected ProgramParseResult variant");
    }
}
