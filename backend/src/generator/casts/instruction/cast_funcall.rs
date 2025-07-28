use super::GeneratorCasts;
use crate::asm::*;
use language::*;


impl GeneratorCasts {
    pub fn cast_funcall(fun_name: &str, args: &[TacVal], dst: &TacVal) -> Vec<AsmInstruction> {
        const ARG_REGISTERS: [AsmReg; 6] = [
            AsmReg::DI, AsmReg::SI, AsmReg::DX,
            AsmReg::CX, AsmReg::R8, AsmReg::R9,
        ];

        let n_reg_args = args.len().min(ARG_REGISTERS.len());
        let n_stack_args = args.len().saturating_sub(n_reg_args) as i32;
        let padding = if n_stack_args % 2 != 0 { 8 } else { 0 };

        let mut instrs = Vec::new();

        if padding > 0 {
            instrs.push(AsmInstruction::AllocateStack(padding));
        }

        for (i, arg) in args.iter().take(n_reg_args).enumerate() {
            let src = convert_operand(arg);
            let dst = AsmOperand::Reg(ARG_REGISTERS[i].clone());
            instrs.push(AsmInstruction::Mov { src, dst });
        }

        for arg in args.iter().skip(n_reg_args).rev() {
            let src = convert_operand(arg);
            match src {
                AsmOperand::Imm(_) | AsmOperand::Reg(_) => {
                    instrs.push(AsmInstruction::Push(src));
                }
                _ => {
                    instrs.push(AsmInstruction::Mov {
                        src: src.clone(),
                        dst: AsmOperand::Reg(AsmReg::AX),
                    });
                    instrs.push(AsmInstruction::Push(AsmOperand::Reg(AsmReg::AX)));
                }
            }
        }

        instrs.push(AsmInstruction::Call(fun_name.to_string()));

        let stack_cleanup = (n_stack_args * 8 + padding) as i32;
        if stack_cleanup > 0 {
            instrs.push(AsmInstruction::DeallocateStack(stack_cleanup));
        }

        instrs.push(AsmInstruction::Mov {
            src: AsmOperand::Reg(AsmReg::AX),
            dst: convert_operand(dst),
        });

        instrs
    }
}