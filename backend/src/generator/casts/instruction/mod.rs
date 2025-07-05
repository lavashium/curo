mod cast_binary;
mod cast_return;
mod cast_unary;
mod cast_jump;

use super::*;
use crate::asm::*;
use language::*;

pub trait InstructionCast {
    fn cast_instruction(&self, instruction: &TacInstruction) -> Vec<AsmInstruction>;
}

impl<'a> InstructionCast for GeneratorCasts<'a> {
    fn cast_instruction(&self, instruction: &TacInstruction) -> Vec<AsmInstruction> {
        #[allow(unused_variables)]
        match instruction {
            instr @ TacInstruction::Return { .. } => self.cast_return(instr),
            instr @ TacInstruction::Unary { .. } => self.cast_unary(instr),
            instr @ TacInstruction::Binary { .. } => self.cast_binary(instr),
            instr @ TacInstruction::Copy { src, dst } => vec![AsmInstruction::new_mov(convert_operand(src), convert_operand(dst))],
            instr @ TacInstruction::Jump { .. } => self.cast_jump(instr),
            instr @ TacInstruction::JumpIfZero { .. } => self.cast_jump(instr),
            instr @ TacInstruction::JumpIfNotZero { .. } => self.cast_jump(instr),
            instr @ TacInstruction::Label( name ) => vec![AsmInstruction::new_label(name.clone())],
        }
    }
}
