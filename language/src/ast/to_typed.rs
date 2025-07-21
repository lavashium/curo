use crate::*;

impl AstProgram {
    pub fn to_typed(&self) -> TypedProgram {
        TypedProgram::new(
            self.functions()
                .iter()
                .map(|f| f.to_typed())
                .collect()
        )
    }
}

impl AstFunctionDeclaration {
    pub fn to_typed(&self) -> TypedFunctionDeclaration {
        TypedFunctionDeclaration::new(
            self.identifier().clone(),
            self.params().clone(),
            self.body().as_ref().map(|b| b.to_typed()),
            AstType::default(),
            *self.span()
        )
    }
}

impl AstBlock {
    pub fn to_typed(&self) -> TypedBlock {
        TypedBlock::new(
            self.block_items()
                .iter()
                .map(|item| item.to_typed())
                .collect(),
            AstType::default(),
            *self.span()
        )
    }
}

impl AstBlockItem {
    pub fn to_typed(&self) -> TypedBlockItem {
        match self {
            AstBlockItem::Statement(stmt) => TypedBlockItem::Statement(stmt.to_typed()),
            AstBlockItem::Declaration(decl) => TypedBlockItem::Declaration(decl.to_typed()),
        }
    }
}

impl AstDeclaration {
    pub fn to_typed(&self) -> TypedDeclaration {
        match self {
            AstDeclaration::FunDecl(f) => TypedDeclaration::FunDecl(f.to_typed()),
            AstDeclaration::VarDecl(v) => TypedDeclaration::VarDecl(v.to_typed()),
        }
    }
}

impl AstVariableDeclaration {
    pub fn to_typed(&self) -> TypedVariableDeclaration {
        TypedVariableDeclaration::new(
            self.identifier().clone(),
            self.init().as_ref().map(|e| e.to_typed()),
            AstType::default(),
            *self.span()
        )
    }
}

impl AstExpression {
    pub fn to_typed(&self) -> TypedExpression {
        match self {
            AstExpression::Constant { constant, span } => {
                TypedExpression::Constant {
                    constant: constant.clone(),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
            AstExpression::Var { identifier, span } => {
                TypedExpression::Var {
                    identifier: identifier.clone(),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
            AstExpression::Unary { operator, operand, span } => {
                TypedExpression::Unary {
                    operator: *operator,
                    operand: Box::new(operand.to_typed()),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
            AstExpression::Binary { operator, left, right, span } => {
                TypedExpression::Binary {
                    operator: *operator,
                    left: Box::new(left.to_typed()),
                    right: Box::new(right.to_typed()),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
            AstExpression::Assignment { left, right, span } => {
                TypedExpression::Assignment {
                    left: Box::new(left.to_typed()),
                    right: Box::new(right.to_typed()),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
            AstExpression::Conditional { condition, then_branch, else_branch, span } => {
                TypedExpression::Conditional {
                    condition: Box::new(condition.to_typed()),
                    then_branch: Box::new(then_branch.to_typed()),
                    else_branch: Box::new(else_branch.to_typed()),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
            AstExpression::FunctionCall { identifier, args, span } => {
                TypedExpression::FunctionCall {
                    identifier: identifier.clone(),
                    args: args.iter().map(|a| Box::new(a.to_typed())).collect(),
                    ty: AstType::default(),
                    span: span.clone(),
                }
            }
        }
    }
}

impl AstStatement {
    pub fn to_typed(&self) -> TypedStatement {
        match self {
            AstStatement::Return { expression, span } => {
                TypedStatement::Return {
                    expression: expression.to_typed(),
                    span: span.clone(),
                }
            }
            AstStatement::Expression { expression, span } => {
                TypedStatement::Expression {
                    expression: expression.to_typed(),
                    span: span.clone(),
                }
            }
            AstStatement::If { condition, then_branch, else_branch, span } => {
                TypedStatement::If {
                    condition: condition.to_typed(),
                    then_branch: Box::new(then_branch.to_typed()),
                    else_branch: else_branch.as_ref().map(|e| Box::new(e.to_typed())),
                    span: span.clone(),
                }
            }
            AstStatement::Compound { block, span } => {
                TypedStatement::Compound {
                    block: block.to_typed(),
                    span: span.clone(),
                }
            }
            AstStatement::Break { label, span } => {
                TypedStatement::Break {
                    label: label.clone(),
                    span: span.clone(),
                }
            }
            AstStatement::Continue { label, span } => {
                TypedStatement::Continue {
                    label: label.clone(),
                    span: span.clone(),
                }
            }
            AstStatement::While { condition, body, label, span } => {
                TypedStatement::While {
                    condition: condition.to_typed(),
                    body: Box::new(body.to_typed()),
                    label: label.clone(),
                    span: span.clone(),
                }
            }
            AstStatement::DoWhile { condition, body, label, span } => {
                TypedStatement::DoWhile {
                    condition: condition.to_typed(),
                    body: Box::new(body.to_typed()),
                    label: label.clone(),
                    span: span.clone(),
                }
            }
            AstStatement::For { for_init, condition, post, body, label, span } => {
                TypedStatement::For {
                    for_init: for_init.to_typed(),
                    condition: condition.as_ref().map(|e| e.to_typed()),
                    post: post.as_ref().map(|e| e.to_typed()),
                    body: Box::new(body.to_typed()),
                    label: label.clone(),
                    span: span.clone(),
                }
            }
            AstStatement::Null => TypedStatement::Null,
        }
    }
}

impl AstForInit {
    pub fn to_typed(&self) -> TypedForInit {
        match self {
            AstForInit::InitDeclaration { decl, span } => {
                TypedForInit::InitDeclaration {
                    decl: decl.to_typed(),
                    span: span.clone(),
                }
            }
            AstForInit::InitExpression { expr, span } => {
                TypedForInit::InitExpression {
                    expr: expr.as_ref().map(|e| e.to_typed()),
                    span: span.clone(),
                }
            }
        }
    }
}
