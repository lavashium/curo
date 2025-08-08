use super::*;
use crate::asm::*;
use crate::*;
use common::*;

impl<'scp, 'ctx> Factory<(), AsmTopLevel> for LegalizerLegalizations<'scp, 'ctx> {
    fn run(top_level: &mut AsmTopLevel, ctx: &mut LegalizerContext<'scp, 'ctx>) {
        match top_level {
            AsmTopLevel::Function { identifier, global, instructions, stack_size } => {
                let aligned_size = ((*stack_size + 15) / 16) * 16;
                
                let mut new_instructions = Vec::new();
                if aligned_size > 0 {
                    new_instructions.push(AsmInstruction::AllocateStack(aligned_size));
                }
                
                for instr in instructions.iter_mut() {
                    if let Some(fixed) = Self::run(instr, ctx) {
                        new_instructions.extend(fixed);
                    } else {
                        new_instructions.push(instr.to_owned());
                    }
                }
                
                *instructions = new_instructions;
            }
            AsmTopLevel::StaticVariable { identifier, global, init } => {}
        }

    }
}