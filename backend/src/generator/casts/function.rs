use super::*;
use crate::asm::*;
use language::*;

pub trait FunctionCast {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction;
}

impl<'a> FunctionCast for GeneratorCasts<'a> {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction {
        let identifier = function.get_identifier().clone();
        let params = function.params();
        let mut instructions = Vec::new();

        let arg_registers = [
            AsmReg::DI, AsmReg::SI, AsmReg::DX,
            AsmReg::CX, AsmReg::R8, AsmReg::R9,
        ];
        for (i, param) in params.iter().enumerate() {
            let dst = AsmOperand::new_pseudo(param.clone());
            let src = if i < arg_registers.len() {
                AsmOperand::Reg(arg_registers[i].clone())
            } else {
                let offset = 16 + 8 * (i - arg_registers.len());
                AsmOperand::Stack(offset as i32)
            };
            instructions.push(AsmInstruction::Mov { src, dst });
        }

        for instr in function.instructions() {
            let mut asm_instrs = self.cast_instruction(instr);
            instructions.append(&mut asm_instrs);
        }

        AsmFunction::new(identifier, instructions)
    }
}