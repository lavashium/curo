use super::*;
use crate::asm::*;
use language::*;
use common::*;

impl Factory<AsmFunction, TacFunction, ()> for GeneratorCasts {
    fn run(function: &mut TacFunction, ctx: &mut ()) -> AsmFunction {
        let identifier = function.get_identifier().clone();
        let mut instructions = Vec::new();

        const ARG_REGISTERS: [AsmReg; 6] = [
            AsmReg::DI, AsmReg::SI, AsmReg::DX,
            AsmReg::CX, AsmReg::R8, AsmReg::R9,
        ];

        for (index, param) in function.params().iter().enumerate() {
            let dst = AsmOperand::new_pseudo(param.clone());

            let src = if index < ARG_REGISTERS.len() {
                AsmOperand::Reg(ARG_REGISTERS[index].clone())
            } else {
                let offset = 16 + 8 * (index - ARG_REGISTERS.len());
                AsmOperand::Stack(offset as i32)
            };

            instructions.push(AsmInstruction::Mov { src, dst });
        }

        for tac_instr in function.instructions_mut() {
            instructions.extend(Self::run(tac_instr, ctx));
        }

        AsmFunction::new(identifier, instructions, 0)
    }
}