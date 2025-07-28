use super::*;
use crate::asm::*;
use crate::*;
use common::*;

impl Factory<(), AsmFunction, LegalizerContext<'_, '_>> for LegalizerLegalizations {
    fn run(function: &mut AsmFunction, ctx: &mut LegalizerContext) {
        let stack_size = function.stack_size();
        let aligned_size = ((stack_size + 15) / 16) * 16;
        
        let mut new_instructions = Vec::new();
        if aligned_size > 0 {
            new_instructions.push(AsmInstruction::AllocateStack(aligned_size));
        }
        
        for instr in function.instructions_mut() {
            if let Some(fixed) = Self::run(instr, ctx) {
                new_instructions.extend(fixed);
            } else {
                new_instructions.push(instr.to_owned());
            }
        }
        
        *function.instructions_mut() = new_instructions;
    }
}