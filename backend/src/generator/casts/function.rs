use super::*;
use crate::asm::*;
use language::*;

pub trait FunctionCast {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction;
}

impl<'a> FunctionCast for GeneratorCasts<'a> {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction {
        let identifier = function.get_identifier();
        let params = function.params();
        let mut instructions = Vec::new();
        
        let arg_registers = vec![AsmReg::DI, AsmReg::SI, AsmReg::DX, AsmReg::CX, AsmReg::R8, AsmReg::R9];
        for (i, param) in params.iter().enumerate() {
            if i < 6 {
                instructions.push(AsmInstruction::new_mov(
                    AsmOperand::new_reg(arg_registers[i].clone()),
                    AsmOperand::new_pseudo(param.clone()),
                ));
            } else {
                let offset = 16 + 8 * (i - 6);
                instructions.push(AsmInstruction::new_mov(
                    AsmOperand::new_stack(offset as i32),
                    AsmOperand::new_pseudo(param.clone()),
                ));
            }
        }
        
        for instruction in function.instructions() {
            let mut asm_instructions = self.cast_instruction(instruction);
            instructions.append(&mut asm_instructions);
        }
        
        AsmFunction::new(identifier, instructions)
    }
}