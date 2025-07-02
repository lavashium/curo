use crate::asm::*;
use super::fixes::*;

pub struct AsmLegalizer {
    stack_size: i32,
}

impl AsmLegalizer {
    pub fn new(stack_size: i32) -> Self {
        Self { stack_size }
    }

    pub fn legalize(&self, program: AsmProgram) -> AsmProgram {
        let mut function = program.function_definition;

        let mut instructions = Vec::with_capacity(function.instructions.len() + 3);
        instructions.push(AsmInstruction::AllocateStack(self.stack_size));

        for instr in function.instructions {
            instructions.extend(self.fix_instruction(instr));
        }

        function.instructions = instructions;

        AsmProgram {
            function_definition: function,
        }
    }

    fn fix_instruction(&self, instr: AsmInstruction) -> Vec<AsmInstruction> {
        if let Some(instructions) = FIXES::try_all(&instr) {
            return instructions;
        } else {
            return vec![instr];
        }
    }
}
