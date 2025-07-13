use accessors::accessors;
use constructors::constructors;

use crate::Span;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstProgram {
    functions: Vec<AstFunctionDeclaration>,
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
pub struct AstBlock {
    block_items: Vec<AstBlockItem>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstDeclaration {
    FunDecl(AstFunctionDeclaration),
    VarDecl(AstVariableDeclaration),
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstVariableDeclaration {
    identifier: String,
    init: Option<AstExpression>,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstFunctionDeclaration {
    identifier: String,
    params: Vec<String>,
    body: Option<AstBlock>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstForInit {
    InitDeclaration(AstVariableDeclaration),
    InitExpression(Option<AstExpression>),
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstStatement {
    Return { 
        expression: AstExpression,
        span: Span,
    },
    Expression {
        expression: AstExpression,
        span: Span,
    },
    If {
        condition: AstExpression,
        then_branch: Box<AstStatement>,
        else_branch: Option<Box<AstStatement>>,
        span: Span,
    },
    Compound {
        block: AstBlock,
        span: Span,
    },
    Break {
        label: String,
        span: Span,
    },
    Continue {
        label: String,
        span: Span,
    },
    While {
        condition: AstExpression,
        body: Box<AstStatement>,
        label: String,
        span: Span,
    },
    DoWhile {
        condition: AstExpression,
        body: Box<AstStatement>,
        label: String,
        span: Span,
    },
    For {
        for_init: AstForInit,
        condition: Option<AstExpression>,
        post: Option<AstExpression>,
        body: Box<AstStatement>,
        label: String,
        span: Span,
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
    },
    FunctionCall {
        identifier: String,
        args: Vec<Box<AstExpression>>,
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
            AstExpression::FunctionCall { span, ..} => *span,
        }
    }
}
