use common::*;
use super::*;

impl Factory<(), TypedStatement, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(stmt: &mut TypedStatement, ctx: &mut AnalyzerContext) {
        match stmt {
            TypedStatement::Return { expression, .. } => TypeCheck::run(expression, ctx),
            TypedStatement::Expression { expression, .. } => TypeCheck::run(expression, ctx),
            
            TypedStatement::If { 
                condition, 
                then_branch, 
                else_branch,  
                .. 
            } => {
                TypeCheck::run(condition, ctx);
                TypeCheck::run(&mut **then_branch, ctx);
                if let Some(else_stmt) = else_branch {
                    TypeCheck::run(&mut **else_stmt, ctx);
                }
            }
            
            TypedStatement::Compound { block, .. } => {
                TypeCheck::run(block, ctx);
            }
            
            TypedStatement::While { 
                condition, 
                body, 
                .. 
            } => {
                TypeCheck::run(condition, ctx);
                TypeCheck::run(&mut **body, ctx);
            }
            
            TypedStatement::DoWhile { 
                condition, 
                body, 
                .. 
            } => {
                TypeCheck::run(&mut **body, ctx);
                TypeCheck::run(condition, ctx);
            }
            
            TypedStatement::For { 
                for_init, 
                condition, 
                post, 
                body, 
                .. 
            } => {
                match for_init {
                    TypedForInit::InitDeclaration { decl, .. } => {
                        TypeCheck::run(decl, ctx);
                    }
                    TypedForInit::InitExpression { expr, .. } => {
                        if let Some(e) = expr {
                            TypeCheck::run(e, ctx);
                        }
                    }
                }
                
                if let Some(cond) = condition {
                    TypeCheck::run(cond, ctx);
                }
                
                if let Some(p) = post {
                    TypeCheck::run(p, ctx);
                }
                
                TypeCheck::run(&mut **body, ctx);
            }
            
            TypedStatement::Null | 
            TypedStatement::Break { .. } | 
            TypedStatement::Continue { .. } => (),
        }
    }
}