use crate::asm::*;
use language::*;

pub fn convert_operand(val: &TacVal) -> AsmOperand {
    match val {
        TacVal::Constant(s) => {
            let parsed = s.parse::<i32>().expect("invalid constant integer");
            AsmOperand::Imm(parsed)
        }
        TacVal::Var(ident) => AsmOperand::Pseudo(ident.clone()),
    }
}

pub fn convert_unary_operator(op: &TacUnaryKind) -> AsmUnaryOperator {
    match op {
        TacUnaryKind::Negate => AsmUnaryOperator::Neg,
        TacUnaryKind::Complement => AsmUnaryOperator::Not,
        TacUnaryKind::Not => todo!(),
    }
}

pub fn convert_binary_operator(op: &TacBinaryKind) -> AsmBinaryOperator {
    match op {
        TacBinaryKind::Add => AsmBinaryOperator::Add,
        TacBinaryKind::Subtract => AsmBinaryOperator::Sub,
        TacBinaryKind::Multiply => AsmBinaryOperator::Mult,
        TacBinaryKind::Remainder => unreachable!(),
        TacBinaryKind::Divide => unreachable!(),
        TacBinaryKind::Equal => todo!(),
        TacBinaryKind::NotEqual => todo!(),
        TacBinaryKind::LessThan => todo!(),
        TacBinaryKind::LessOrEqual => todo!(),
        TacBinaryKind::GreaterThan => todo!(),
        TacBinaryKind::GreaterOrEqual => todo!(),
    }
}

pub fn convert_to_cond_code(op: &TacBinaryKind) -> AsmCondCode {
    match op {
        TacBinaryKind::Equal          => AsmCondCode::E,
        TacBinaryKind::NotEqual       => AsmCondCode::NE,
        TacBinaryKind::LessThan       => AsmCondCode::L,
        TacBinaryKind::LessOrEqual    => AsmCondCode::LE,
        TacBinaryKind::GreaterThan    => AsmCondCode::G,
        TacBinaryKind::GreaterOrEqual => AsmCondCode::GE,
        _ => unreachable!()
    }
}
