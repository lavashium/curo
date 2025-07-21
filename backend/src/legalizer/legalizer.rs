use super::*;
use super::fixes::*;
use crate::asm::*;
use accessors::accessors;
use constructors::constructors;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct AsmLegalizer<'scp> {
    source_asm: &'scp mut AsmProgram,
}

impl<'scp> AsmLegalizer<'scp> {
    #[zawarudo(label = "Assembly Legalizer")]
    pub fn legalize(&self, ctx: &mut LegalizerContext) -> AsmProgram {
        let mut program_instructions = Vec::new();

        for mut function in self.source_asm.get_function_definitions() {
            let mut instructions = Vec::with_capacity(function.instructions().len() + 3);
            instructions.push(AsmInstruction::AllocateStack(ctx.stack_size));

            for instr in function.get_instructions() {
                instructions.extend(self.fix_instruction(instr));
            }

            function.set_instructions(instructions);
            program_instructions.push(function)
        }

        AsmProgram::new(program_instructions)
    }

    fn fix_instruction(&self, instr: AsmInstruction) -> Vec<AsmInstruction> {
        if let Some(instructions) = FIXES::try_all(&instr) {
            return instructions;
        } else {
            return vec![instr];
        }
    }
}
