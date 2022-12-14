use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, Table, Vector};
use mpl_candy_guard::instructions::unwrap;
use plerkle_serialization::{
    root_as_compiled_instruction, CompiledInstruction, CompiledInstructionArgs,
    CompiledInstructionBuilder, InnerInstructions, Pubkey, TransactionInfo,
};
use std::collections::{HashSet, VecDeque};

pub type IxPair<'a> = (Pubkey, CompiledInstruction<'a>);

pub struct InstructionBundle<'a> {
    pub txn_id: &'a str,
    pub program: Pubkey,
    pub instruction: Option<CompiledInstruction<'a>>,
    pub inner_ix: Option<Vec<IxPair<'a>>>,
    pub keys: &'a [Pubkey],
    pub slot: u64,
}

impl<'a> Default for InstructionBundle<'a> {
    fn default() -> Self {
        InstructionBundle {
            txn_id: "",
            program: Pubkey::new(&[0; 32]),
            instruction: None,
            inner_ix: None,
            keys: &[],
            slot: 0,
        }
    }
}

pub fn order_instructions<'a, 'b>(
    programs: HashSet<&'b [u8]>,
    transaction_info: &'a TransactionInfo<'a>,
) -> VecDeque<(IxPair<'a>, Option<Vec<IxPair<'a>>>)> {
    let mut ordered_ixs: VecDeque<(IxPair, Option<Vec<IxPair>>)> = VecDeque::new();
    // Get inner instructions.
    let inner_ix_list = transaction_info.inner_instructions();

    // Get outer instructions.
    let outer_instructions = match transaction_info.outer_instructions() {
        None => {
            println!("outer instructions deserialization error");
            return ordered_ixs;
        }
        Some(instructions) => instructions,
    };

    // Get account keys.
    let keys = match transaction_info.account_keys() {
        None => {
            println!("account_keys deserialization error");
            return ordered_ixs;
        }
        Some(keys) => keys.iter().collect::<Vec<_>>(),
    };

    for (i, instruction) in outer_instructions.iter().enumerate() {
        let program_id = keys.get(instruction.program_id_index() as usize).unwrap();
        println!("program_id: {:?}", bs58::encode(program_id.0).into_string());
        let outer: IxPair = (**program_id, instruction);
        let inner: Option<Vec<IxPair>> =
            inner_ix_list.and_then(|x| fill_inner(x.iter(), &keys, i as u8));
        if let Some(inner_ix) = &inner {
            for (key, ix) in inner_ix {
                let inner_program_id = key;
                if programs.get(inner_program_id.0.as_ref()).is_some() {
                    let new_inner_list = inner_ix.clone();
                    println!(
                        "\t\t hoisted {:?}",
                        bs58::encode(inner_program_id.0).into_string()
                    );

                    let local_inner = (*inner_program_id, *ix);
                    ordered_ixs.push_back((local_inner, Some(new_inner_list)));
                }
            }
        }
        if programs.get(program_id.0.as_ref()).is_some() {
            ordered_ixs.push_back((outer, inner));
        }
    }
    ordered_ixs
}

fn fill_inner<'a>(
    ixes: impl Iterator<Item = InnerInstructions<'a>>,
    keys: &Vec<&Pubkey>,
    index: u8,
) -> Option<Vec<IxPair<'a>>> {
    get_inner_ixs(ixes, index).map(|inner_ixs| {
        let mut inner_list: VecDeque<IxPair> = VecDeque::new();
        for inner_ix_instance in inner_ixs.instructions().unwrap() {
            let inner_program_id = keys
                .get(inner_ix_instance.program_id_index() as usize)
                .unwrap();
            println!(
                "\t\t inner {:?}",
                bs58::encode(inner_program_id.0).into_string()
            );
            inner_list.push_front((**inner_program_id, inner_ix_instance));
        }
        inner_list.into()
    })
}

fn get_hoisted_instructions() {}

fn get_inner_ixs<'a>(
    inner_ixs: impl Iterator<Item = InnerInstructions<'a>>,
    outer_index: u8,
) -> Option<InnerInstructions<'a>> {
    inner_ixs.filter(|inn| inn.index() == outer_index).next()
}
