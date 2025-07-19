use super::GeneratorCasts;
use crate::asm::AsmInstruction::*;
use crate::asm::AsmReg::*;
use crate::asm::*;
use language::*;

impl<'a> GeneratorCasts<'a> {
    pub fn cast_funcall(&self, fun_name: &str, args: &[TacVal], dst: &TacVal) -> Vec<AsmInstruction> {
        let arg_registers = vec![DI, SI, DX, CX, R8, R9];
        let n_reg_args = std::cmp::min(6, args.len());
        let n_stack_args = args.len() - n_reg_args;
        let padding = if n_stack_args % 2 == 0 { 8 } else { 0 };
        let mut instructions = Vec::new();

        if padding != 0 {
            instructions.push(AllocateStack(padding));
        }

        for i in 0..n_reg_args {
            let src = convert_operand(&args[i]);
            instructions.push(Mov {
                src,
                dst: AsmOperand::new_reg(arg_registers[i].clone()),
            });
        }

        for i in (n_reg_args..args.len()).rev() {
            let src = convert_operand(&args[i]);
            match src {
                AsmOperand::Imm(_) | AsmOperand::Reg(_) => {
                    instructions.push(Push(src));
                }
                _ => {
                    instructions.push(Mov {
                        src,
                        dst: AsmOperand::new_reg(AX),
                    });
                    instructions.push(Push(AsmOperand::new_reg(AX)));
                }
            }
        }

        instructions.push(Call(fun_name.to_string()));

        let bytes_to_remove = (n_stack_args * 8) + padding as usize;
        if bytes_to_remove != 0 {
            instructions.push(DeallocateStack(bytes_to_remove as i32));
        }

        instructions.push(Mov {
            src: AsmOperand::new_reg(AX),
            dst: convert_operand(dst),
        });

        instructions
    }
}