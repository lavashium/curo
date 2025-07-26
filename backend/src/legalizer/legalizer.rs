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
        let mut program = Vec::new();
        for (idx, mut function) in self.source_asm.get_function_definitions().into_iter().enumerate() {
            let mut insns = Vec::new();
            let raw = ctx.stack_sizes[idx];
            let aligned = ((raw + 15) / 16) * 16;

            if aligned > 0 {
                insns.push(AsmInstruction::AllocateStack(aligned));
            }
            for instr in function.instructions() {
                if let Some(repl) = FIXES::try_all(instr) {
                    insns.extend(repl);
                } else {
                    insns.push(instr.clone());
                }
            }
            function.set_instructions(insns);
            program.push(function);
        }
        AsmProgram::new(program)
    }
}
