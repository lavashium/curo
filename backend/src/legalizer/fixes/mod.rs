mod binop;
mod mov;
mod unary;
mod compare;

use binop::*;
use mov::*;
use unary::*;
use compare::*;

use crate::asm::*;

macro_rules! auto_nest {
    () => {
        ()
    };
    ($head:ty $(, $tail:ty)* $(,)?) => {
        ($head, auto_nest!($($tail),*))
    };
}

pub trait LegalizerChain {
    fn try_all(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>>;
}

impl LegalizerChain for () {
    fn try_all(_: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        None
    }
}

impl<Head: Legalizer, Tail: LegalizerChain> LegalizerChain for (Head, Tail) {
    fn try_all(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        Head::legalize(instr).or_else(|| Tail::try_all(instr))
    }
}

pub type FIXES = auto_nest!(
    MovLegalizer,
    UnaryLegalizer, 
    BinopLegalizer,
    CompareLegalizer,
);

pub trait Legalizer {
    fn legalize(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>>;
}

pub fn both_stack_operands(src: &AsmOperand, dst: &AsmOperand) -> bool {
    is_stack_operand(src) && is_stack_operand(dst)
}

pub fn is_stack_operand(op: &AsmOperand) -> bool {
    matches!(op, AsmOperand::Stack(_))
}
