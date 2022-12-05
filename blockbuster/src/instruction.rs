use flatbuffers::{ForwardsUOffset, Table, Vector, FlatBufferBuilder};
use mpl_candy_guard::instructions::unwrap;
use plerkle_serialization::{CompiledInstruction, InnerInstructions, Pubkey, TransactionInfo, CompiledInstructionBuilder, root_as_compiled_instruction, CompiledInstructionArgs};
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
        let outer: IxPair = (**program_id, instruction);

        let inner: Option<Vec<IxPair>> = get_inner_ixs(inner_ix_list, i).map(|inner_ixs| {
            let mut inner_list: VecDeque<IxPair> = VecDeque::new();
            for inner_ix_instance in inner_ixs.instructions().unwrap() {
                let inner_program_id = keys
                    .get(inner_ix_instance.program_id_index() as usize)
                    .unwrap();
                inner_list.push_front((**inner_program_id, inner_ix_instance));
                if programs.get(inner_program_id.0.as_ref()).is_some() {
                    println!("\t\t added {:?}", inner_program_id);
                    let mut new_inner_list = inner_list.clone();
                    new_inner_list.pop_front();
                    let inner = (**inner_program_id, inner_ix_instance);
                    ordered_ixs.push_back((inner, Some(new_inner_list.into())));
                }
            }
            inner_list.into()
        });
        if programs.get(program_id.0.as_ref()).is_some() {
            ordered_ixs.push_back((outer, inner));
        }
    }

    ordered_ixs
}

fn get_inner_ixs<'a>(
    inner_ixs: Option<Vector<'a, ForwardsUOffset<InnerInstructions<'_>>>>,
    outer_index: usize,
) -> Option<InnerInstructions<'a>> {
    match inner_ixs {
        Some(inner_ix_list) => {
            for inner_ixs in inner_ix_list {
                if inner_ixs.index() == (outer_index as u8) {
                    return Some(inner_ixs);
                }
            }
            None
        }
        None => None,
    }
}
