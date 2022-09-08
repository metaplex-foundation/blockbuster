use plerkle_serialization::transaction_info_generated::transaction_info::{
    CompiledInstruction,
    Pubkey,
};
use flatbuffers::{Vector, ForwardsUOffset};

pub type IxPair<'a> = (
    Pubkey,
    CompiledInstruction<'a>,
);


pub struct InstructionBundle<'a> {
    pub txn_id: String,
    pub program: Pubkey,
    pub instruction: CompiledInstruction<'a>,
    pub inner_ix: Option<Vec<IxPair<'a>>>,
    pub keys: &'a [Pubkey],
    pub slot: u64,
}
