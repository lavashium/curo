use super::*;

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