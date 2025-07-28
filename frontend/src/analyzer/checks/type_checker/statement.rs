use common::*;
use super::*;

impl Factory<(), TypedStatement, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(stmt: &mut TypedStatement, ctx: &mut AnalyzerContext) {
        match stmt {
            TypedStatement::Return { expression, .. } => Self::run(expression, ctx),
            TypedStatement::Expression { expression, .. } => Self::run(expression, ctx),
            
            TypedStatement::If { 
                condition, 
                then_branch, 
                else_branch,  
                .. 
            } => {
                Self::run(condition, ctx);
                Self::run_box(then_branch, ctx);
                Self::run_option_box(else_branch, ctx);
            }
            
            TypedStatement::Compound { block, .. } => {
                Self::run(block, ctx);
            }
            
            TypedStatement::While { 
                condition, 
                body, 
                .. 
            } => {
                Self::run(condition, ctx);
                Self::run_box(body, ctx);
            }
            
            TypedStatement::DoWhile { 
                condition, 
                body, 
                .. 
            } => {
                Self::run_box(body, ctx);
                Self::run(condition, ctx);
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
                        Self::run(decl, ctx);
                    }
                    TypedForInit::InitExpression { expr, .. } => {
                        Self::run_option(expr, ctx);
                    }
                }
                
                Self::run_option(condition, ctx);
                Self::run_option(post, ctx);
                Self::run_box(body, ctx);
            }
            
            TypedStatement::Null | 
            TypedStatement::Break { .. } | 
            TypedStatement::Continue { .. } => (),
        }
    }
}