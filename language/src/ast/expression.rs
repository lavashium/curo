use constructors::constructors;

use crate::Span;
use super::*;

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstExpression {
    Constant {
        constant: String,
        span: Span
    },
    Var {
        identifier: String,
        span: Span
    },
    Unary {
        operator: AstUnaryKind,
        operand: Box<AstExpression>,
        span: Span
    },
    Binary {
        operator: AstBinaryKind,
        left: Box<AstExpression>,
        right: Box<AstExpression>,
        span: Span
    },
    Assignment {
        left: Box<AstExpression>,
        right: Box<AstExpression>,
        span: Span
    },
    Conditional {
        condition: Box<AstExpression>,
        then_branch: Box<AstExpression>,
        else_branch: Box<AstExpression>,
        span: Span
    },
    FunctionCall {
        identifier: String,
        args: Vec<Box<AstExpression>>,
        span: Span
    }
}

