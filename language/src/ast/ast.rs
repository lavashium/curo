use accessors::accessors;
use constructors::constructors;

use crate::Span;
use super::*;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstProgram {
    declarations: Vec<AstDeclaration>,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstFunctionDeclaration {
    identifier: String,
    params: Vec<String>,
    body: Option<AstBlock>,
    storage_class: Option<AstStorageClass>,
    span: Span,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstBlock {
    block_items: Vec<AstBlockItem>,
    span: Span,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBlockItem {
    Statement(AstStatement),
    Declaration(AstDeclaration),
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
    storage_class: Option<AstStorageClass>,
    span: Span,
}

