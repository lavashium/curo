use super::*;
use crate::asm::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<AsmTopLevel, TacTopLevel> for GeneratorCasts<'scp, 'ctx> {
    fn run(top_level: &mut TacTopLevel, ctx: &mut GeneratorContext<'scp, 'ctx>) -> AsmTopLevel {
        match top_level {
            TacTopLevel::Function { identifier, global, params, instructions } => {
                let mut asm_instructions = Vec::new();

                const ARG_REGISTERS: [AsmReg; 6] = [
                    AsmReg::DI, AsmReg::SI, AsmReg::DX,
                    AsmReg::CX, AsmReg::R8, AsmReg::R9,
                ];

                for (index, param) in params.iter().enumerate() {
                    let dst = AsmOperand::new_pseudo(param.clone());

                    let src = if index < ARG_REGISTERS.len() {
                        AsmOperand::Reg(ARG_REGISTERS[index].clone())
                    } else {
                        let offset = 16 + 8 * (index - ARG_REGISTERS.len());
                        AsmOperand::Stack(offset as i32)
                    };

                    asm_instructions.push(AsmInstruction::new_mov(src, dst));
                }

                for tac_instr in instructions.iter_mut() {
                    asm_instructions.extend(GeneratorCasts::run(tac_instr, ctx));
                }

                AsmTopLevel::Function {
                    identifier: identifier.clone(),
                    global: *global,
                    instructions: asm_instructions,
                    stack_size: 0,
                }
            }

            TacTopLevel::StaticVariable { identifier, global, init } => {
                AsmTopLevel::StaticVariable {
                    identifier: identifier.clone(),
                    global: *global,
                    init: init.clone(),
                }
            }
        }
    }
}
