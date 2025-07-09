use accessors::accessors;
use constructors::constructors;

use crate::Span;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstProgram {
    function_definition: AstFunction,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstFunction {
    name: String,
    body: Vec<AstBlockItem>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBlockItem {
    Statement(AstStatement),
    Declaration(AstDeclaration),
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstDeclaration {
    name: String,
    init: Option<AstExpression>,
    span: Span,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstStatement {
    Return { 
        expression: AstExpression,
    },
    Expression {
        expression: AstExpression,
    },
    If {
        condition: AstExpression,
        then_branch: Box<AstStatement>,
        else_branch: Option<Box<AstStatement>>,
    },
    Null,
}

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
    }
}

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AstUnaryKind {
    Complement,
    Negate,
    Not,
}

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AstBinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

impl AstExpression {
    pub fn get_span(&self) -> Span {
        match self {
            AstExpression::Constant { span, .. } => *span,
            AstExpression::Var { span, .. } => *span,
            AstExpression::Unary { span, .. } => *span,
            AstExpression::Binary { span, .. } => *span,
            AstExpression::Assignment { span, .. } => *span,
            AstExpression::Conditional { span, .. } => *span,
        }
    }
}
