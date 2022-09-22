extern crate core;

use crate::helpers::{build_account_update, random_list, random_pubkey};
use blockbuster::{
    error::BlockbusterError,
    programs::candy_machine::{
        state::{
            CandyMachine as CandyMachineAccountType,
            CandyMachineData as CandyMachineDataAccountType, Creator, EndSettingType, EndSettings,
            GatekeeperConfig, HiddenSettings, WhitelistMintMode, WhitelistMintSettings,
        },
        CandyMachineAccountData::CandyMachine,
    },
};
use blockbuster::{
    program_handler::ProgramParser,
    programs::{
        candy_machine::{candy_machine_id, CandyMachineParser, CANDY_MACHINE_DISCRIMINATOR},
        ProgramParseResult,
    },
};
use borsh::BorshSerialize;
use flatbuffers::FlatBufferBuilder;
use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfo;

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

    // Create a `ReplicaAccountInfo` to store the account update.
    let replica_account_info = ReplicaAccountInfo {
        pubkey: &random_pubkey().to_bytes()[..],
        lamports: 1,
        owner: &random_pubkey().to_bytes()[..],
        executable: false,
        rent_epoch: 1000,
        data: &data,
        write_version: 1,
    };

    // Flatbuffer serialize the `ReplicaAccountInfo` into
    let mut fbb = FlatBufferBuilder::new();
    let account_info = build_account_update(&mut fbb, &replica_account_info, 0, false)
        .expect("Could not build account update");

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
fn test_unkown_discriminator_fails() {
    // Borsh serialize the CandyMachine discriminator.
    let mut data = CANDY_MACHINE_DISCRIMINATOR.to_vec();

    // Corrupt the discriminator.
    data[0] = 0;

    // Create a `ReplicaAccountInfo` to store the account update.
    let replica_account_info = ReplicaAccountInfo {
        pubkey: &random_pubkey().to_bytes()[..],
        lamports: 1,
        owner: &random_pubkey().to_bytes()[..],
        executable: false,
        rent_epoch: 1000,
        data: &data,
        write_version: 1,
    };

    // Flatbuffer serialize the `ReplicaAccountInfo` into
    let mut fbb = FlatBufferBuilder::new();
    let account_info = build_account_update(&mut fbb, &replica_account_info, 0, false)
        .expect("Could not build account update");

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

    // Create a `ReplicaAccountInfo` to store the account update.
    let replica_account_info = ReplicaAccountInfo {
        pubkey: &random_pubkey().to_bytes()[..],
        lamports: 1,
        owner: &random_pubkey().to_bytes()[..],
        executable: false,
        rent_epoch: 1000,
        data: &data,
        write_version: 1,
    };

    // Flatbuffer serialize the `ReplicaAccountInfo` into
    let mut fbb = FlatBufferBuilder::new();
    let account_info = build_account_update(&mut fbb, &replica_account_info, 0, false)
        .expect("Could not build account update");

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
