mod cast_return;
mod cast_unary;
mod cast_binary;

use crate::asm::*;
use language::*;
use super::*;

pub trait InstructionCast {
    fn cast_instruction(&self, instruction: &TacInstruction) -> Vec<AsmInstruction>;
}

impl<'a> InstructionCast for GeneratorCasts<'a> {
    fn cast_instruction(&self, instruction: &TacInstruction) -> Vec<AsmInstruction> {
        match instruction {
            instr @ TacInstruction::Return { .. } => self.cast_return(instr),
            instr @ TacInstruction::Unary  { .. } => self.cast_unary(instr),
            instr @ TacInstruction::Binary { .. } => self.cast_binary(instr),
        }
    }
}