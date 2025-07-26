use super::GeneratorCasts;
use crate::asm::*;
use language::*;


impl<'a> GeneratorCasts<'a> {
    pub fn cast_funcall(&self, fun_name: &str, args: &[TacVal], dst: &TacVal) -> Vec<AsmInstruction> {
        let arg_registers = [
            AsmReg::DI, AsmReg::SI, AsmReg::DX,
            AsmReg::CX, AsmReg::R8, AsmReg::R9,
        ];
        let n_reg_args = args.len().min(arg_registers.len());
        let n_stack_args = args.len().saturating_sub(n_reg_args);
        let padding = if n_stack_args % 2 != 0 { 8 } else { 0 };
        let mut instrs = Vec::new();

        if padding != 0 {
            instrs.push(AsmInstruction::AllocateStack(padding));
        }
        for i in 0..n_reg_args {
            let src = convert_operand(&args[i]);
            let dst_reg = AsmOperand::Reg(arg_registers[i].clone());
            instrs.push(AsmInstruction::Mov { src, dst: dst_reg });
        }
        for i in (n_reg_args..args.len()).rev() {
            let src = convert_operand(&args[i]);
            match &src {
                AsmOperand::Imm(_) | AsmOperand::Reg(_) => instrs.push(AsmInstruction::Push(src)),
                _ => {
                    instrs.push(AsmInstruction::Mov { src: src.clone(), dst: AsmOperand::Reg(AsmReg::AX) });
                    instrs.push(AsmInstruction::Push(AsmOperand::Reg(AsmReg::AX)));
                }
            }
        }
        instrs.push(AsmInstruction::Call(fun_name.to_string()));
        let cleanup = (n_stack_args * 8) as i32 + padding;
        if cleanup > 0 {
            instrs.push(AsmInstruction::DeallocateStack(cleanup as i32));
        }
        instrs.push(AsmInstruction::Mov { src: AsmOperand::Reg(AsmReg::AX), dst: convert_operand(dst) });
        instrs
    }
}