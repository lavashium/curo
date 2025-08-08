mod cast_binary;
mod cast_return;
mod cast_unary;
mod cast_jump;
mod cast_funcall;

use super::*;
use crate::asm::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<AsmInstruction>, TacInstruction> for GeneratorCasts<'scp, 'ctx> {
    fn run(instruction: &mut TacInstruction, _: &mut GeneratorContext<'scp, 'ctx>) -> Vec<AsmInstruction> {
        #[allow(unused_variables)]
        match instruction {
            TacInstruction::Return { .. } => Self::cast_return(instruction),
            TacInstruction::Unary { .. } => Self::cast_unary(instruction),
            TacInstruction::Binary { .. } => Self::cast_binary(instruction),
            TacInstruction::Jump { .. } => Self::cast_jump(instruction),
            TacInstruction::JumpIfZero { .. } => Self::cast_jump(instruction),
            TacInstruction::JumpIfNotZero { .. } => Self::cast_jump(instruction),
            TacInstruction::Copy { src, dst } => vec![AsmInstruction::new_mov(convert_operand(src), convert_operand(dst))],
            TacInstruction::Label( name ) => vec![AsmInstruction::new_label(name.clone())],
            TacInstruction::FunCall {
                fun_name,
                args,
                dst,
            } => Self::cast_funcall(fun_name, args, dst),
        }
    }
}
