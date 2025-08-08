use accessors::accessors;
use constructors::constructors;

use crate::AstType;
use super::*;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedProgram {
    declarations: Vec<TypedDeclaration>,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedFunctionDeclaration {
    identifier: String,
    params: Vec<String>,
    body: Option<TypedBlock>,
    storage_class: Option<AstStorageClass>,
    ty: AstType,
    span: Span,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedBlock {
    block_items: Vec<TypedBlockItem>,
    ty: AstType,
    span: Span,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedBlockItem {
    Statement(TypedStatement),
    Declaration(TypedDeclaration),
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedDeclaration {
    FunDecl(TypedFunctionDeclaration),
    VarDecl(TypedVariableDeclaration),
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedVariableDeclaration {
    identifier: String,
    init: Option<TypedExpression>,
    storage_class: Option<AstStorageClass>,
    ty: AstType, 
    span: Span
}