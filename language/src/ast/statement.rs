use constructors::constructors;

use crate::Span;
use super::*;

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstForInit {
    InitDeclaration {
        decl: AstVariableDeclaration,
        span: Span,
    },
    InitExpression {
        expr: Option<AstExpression>,
        span: Span,
    },
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