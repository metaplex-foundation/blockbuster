pub enum Program {
    Bubblegum {
        parser: bubblegum::BubblegumParser,
        instruction_result: BubblegumInstruction,
        account_result: (),
    }
}

impl ProgramParser for Program {}


// pub enum ParsedPrograms<'a> {
//     BubblegumParser(&'a bubblegum::BubblegumParser),
//     GummyrollParser(&'a )
// }
// 
// pub enum ParsedProgramResult {
//     Bubblegum(BubblegumInstruction),
//     GummyRoll(GummyrollParser)
// }
// 
// 
// impl ParsedProgramResult {
// 
// }
