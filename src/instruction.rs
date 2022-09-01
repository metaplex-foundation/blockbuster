use plerkle_serialization::transaction_info_generated::transaction_info::{
    CompiledInstruction,
    Pubkey,
};
use flatbuffers::{Vector, ForwardsUOffset};

pub type IxPair<'a> = (
    Pubkey,
    CompiledInstruction<'a>,
);


pub struct InstructionBundle<'a, 'b> {
    pub txn_id: String,
    pub instruction: CompiledInstruction<'a>,
    pub inner_ix: Option<Vec<IxPair<'a>>>,
    pub keys: Vector<'b, ForwardsUOffset<Pubkey>>,
    pub instruction_logs: Vec<&'b str>,
    pub slot: u64,
}
