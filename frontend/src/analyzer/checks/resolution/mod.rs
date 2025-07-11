mod program;
mod function;
mod block;
mod block_item;
mod declaration;
mod statement;
mod expression;

use program::*;
use function::*;
use block::*;
use block_item::*;
use declaration::*;
use statement::*;
use expression::*;

use std::collections::HashMap;
use common::*;
use language::*;
use super::*;
use constructors::constructors;
use accessors::accessors;

#[derive(Clone)]
#[constructors]
#[accessors]
pub struct VariableInfo {
    unique_name: String,
    from_current_block: bool,
}

pub type VariableMap = HashMap<String, VariableInfo>;

pub struct VariableResolutionCheck;

impl SemanticCheck for VariableResolutionCheck {
    fn analyze(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        let mut map = VariableMap::new();
        resolve_program(ast, ctx, &mut map)
    }
}

pub fn copy_variable_map(src: &VariableMap) -> VariableMap {
    src.iter().map(|(k,v)| {
        (k.clone(), VariableInfo {
            unique_name: v.unique_name.clone(),
            from_current_block: false,
        })
    }).collect()
}