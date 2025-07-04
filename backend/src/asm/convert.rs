use language::*;
use crate::asm::*;

pub fn convert_operand(val: &TacVal) -> AsmOperand {
    match val {
        TacVal::Constant(s) => {
            let parsed = s.parse::<i32>().expect("invalid constant integer");
            AsmOperand::Imm(parsed)
        }
        TacVal::Var(ident) => AsmOperand::Pseudo(ident.clone()),
    }
}

pub fn convert_unary_operator(op: &UnaryKind) -> AsmUnaryOperator {
    match op {
        UnaryKind::Negate => AsmUnaryOperator::Neg,
        UnaryKind::Complement => AsmUnaryOperator::Not,
    }
}

pub fn convert_binary_operator(op: &BinaryKind) -> AsmBinaryOperator {
    match op {
        BinaryKind::Add => AsmBinaryOperator::Add,
        BinaryKind::Subtract => AsmBinaryOperator::Sub,
        BinaryKind::Multiply => AsmBinaryOperator::Mult,
        BinaryKind::Remainder => unreachable!(),
        BinaryKind::Divide => unreachable!(),
    }
}