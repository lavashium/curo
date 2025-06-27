use crate::asm::*;

pub trait ToAsmString {
    fn to_asm_string(&self, indent_level: usize) -> String;
}

impl ToAsmString for AsmProgram {
    fn to_asm_string(&self, indent_level: usize) -> String {
        let mut output = String::new();
        output.push_str(&self.function_definition.to_asm_string(indent_level));
        output.push_str("\n.section .note.GNU-stack,\"\",@progbits\n");
        output
    }
}

impl ToAsmString for AsmFunction {
    fn to_asm_string(&self, indent_level: usize) -> String {
        let indent = " ".repeat(indent_level * 4);
        let mut output = String::new();
        output.push_str(&format!("{}.globl {}\n", indent, self.name));
        output.push_str(&format!("{}:\n", self.name));
        for instr in &self.instructions {
            output.push_str(&format!("{}{}\n", indent, instr.to_asm_string(indent_level + 1)));
        }
        output
    }
}

impl ToAsmString for AsmInstruction {
    fn to_asm_string(&self, _indent_level: usize) -> String {
        match self {
            AsmInstruction::Mov { source, dest } => {
                format!("movl {}, {}", source.to_asm_string(0), dest.to_asm_string(0))
            }
            AsmInstruction::Ret => "ret".to_string(),
        }
    }
}

impl ToAsmString for AsmOperand {
    fn to_asm_string(&self, _indent_level: usize) -> String {
        match self {
            AsmOperand::Register => "%eax".to_string(),
            AsmOperand::Imm(i) => format!("${}", i),
        }
    }
}
