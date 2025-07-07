use super::fixes::*;
use crate::asm::*;
use accessors::accessors;
use constructors::constructors;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct AsmLegalizer {
    stack_size: i32,
}

impl AsmLegalizer {
    #[zawarudo(label = "Assembly Legalizer")]
    pub fn legalize(&self, program: AsmProgram) -> AsmProgram {
        let mut function = program.get_function_definition();

        let mut instructions = Vec::with_capacity(function.instructions().len() + 3);
        instructions.push(AsmInstruction::AllocateStack(self.stack_size));

        for instr in function.get_instructions() {
            instructions.extend(self.fix_instruction(instr));
        }

        function.set_instructions(instructions);

        AsmProgram::new(function)
    }

    fn fix_instruction(&self, instr: AsmInstruction) -> Vec<AsmInstruction> {
        if let Some(instructions) = FIXES::try_all(&instr) {
            return instructions;
        } else {
            return vec![instr];
        }
    }
}
