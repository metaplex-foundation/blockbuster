use flatbuffers::Table;
use blockbuster::program_handler::ProgramParser;
use blockbuster::programs::bubblegum::BubblegumParser;
pub use mpl_bubblegum::{
    id as program_id,
};
use blockbuster::instruction::InstructionBundle;
use blockbuster::program_handler::ParseResult;
use blockbuster::programs::ProgramParseResult;
use plerkle_serialization::{CompiledInstruction, Pubkey};

mod helpers;

#[test]
fn test_setup() {
    let subject = BubblegumParser{};
    assert_eq!(subject.key(), program_id());
    assert!(subject.key_match(&program_id()));
}


#[test]
fn test_basic_success_parsing() {
    let subject = BubblegumParser{};

    let bundle = InstructionBundle{
        txn_id: "",
        program: Pubkey(program_id().to_bytes()),
        instruction: CompiledInstruction{
            _tab: Table {}
        },
        inner_ix: None,
        keys: &[],
        slot: 0
    };
    let result = subject.handle_instruction(&bundle);


}
