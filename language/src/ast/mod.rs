pub mod ast;
pub mod typed;
pub mod expression;
pub mod statement;
pub mod opkind;
pub mod to_typed;

pub use ast::*;
pub use typed::*;
pub use expression::*;
pub use statement::*;
pub use opkind::*;

use crate::Span;

impl AstExpression {
    pub fn get_span(&self) -> Span {
        match self {
            AstExpression::Constant       { span, .. }
            | AstExpression::Var          { span, .. }
            | AstExpression::Unary        { span, .. }
            | AstExpression::Binary       { span, .. }
            | AstExpression::Assignment   { span, .. }
            | AstExpression::Conditional  { span, .. }
            | AstExpression::FunctionCall { span, .. } => *span
        }
    }
}

impl AstStatement {
    pub fn get_span(&self) -> Span {
        match self {
            AstStatement::Return       { span, .. }
            | AstStatement::Expression { span, .. }
            | AstStatement::If         { span, .. }
            | AstStatement::Compound   { span, .. }
            | AstStatement::Break      { span, .. }
            | AstStatement::Continue   { span, .. }
            | AstStatement::While      { span, .. }
            | AstStatement::DoWhile    { span, .. }
            | AstStatement::For        { span, .. } => *span,
            AstStatement::Null                      =>  Span::default(),
        }
    }
}

impl AstDeclaration {
    pub fn get_span(&self) -> Span {
        match self {
            AstDeclaration::FunDecl(fun) => fun.get_span(),
            AstDeclaration::VarDecl(var) => var.get_span(),
        }
    }
}

impl AstBlockItem {
    pub fn get_span(&self) -> Span {
        match self {
            AstBlockItem::Declaration(decl) => decl.get_span(),
            AstBlockItem::Statement(stmt)   => stmt.get_span()
        }
    }
}


impl TypedExpression {
    pub fn get_span(&self) -> Span {
        match self {
            TypedExpression::Constant       { span, .. }
            | TypedExpression::Var          { span, .. }
            | TypedExpression::Unary        { span, .. }
            | TypedExpression::Binary       { span, .. }
            | TypedExpression::Assignment   { span, .. }
            | TypedExpression::Conditional  { span, .. }
            | TypedExpression::FunctionCall { span, .. } => *span
        }
    }
}

impl TypedStatement {
    pub fn get_span(&self) -> Span {
        match self {
            TypedStatement::Return       { span, .. }
            | TypedStatement::Expression { span, .. }
            | TypedStatement::If         { span, .. }
            | TypedStatement::Compound   { span, .. }
            | TypedStatement::Break      { span, .. }
            | TypedStatement::Continue   { span, .. }
            | TypedStatement::While      { span, .. }
            | TypedStatement::DoWhile    { span, .. }
            | TypedStatement::For        { span, .. } => *span,
            TypedStatement::Null                      =>  Span::default(),
        }
    }
}

impl TypedDeclaration {
    pub fn get_span(&self) -> Span {
        match self {
            TypedDeclaration::FunDecl(fun) => fun.get_span(),
            TypedDeclaration::VarDecl(var) => var.get_span(),
        }
    }
}

impl TypedBlockItem {
    pub fn get_span(&self) -> Span {
        match self {
            TypedBlockItem::Declaration(decl) => decl.get_span(),
            TypedBlockItem::Statement(stmt)   => stmt.get_span()
        }
    }
}