mod program;
mod function_declaration;
mod block;
mod block_item;
mod statement;
mod variable_declaration;
mod expression;

use common::*;
use language::*;
use crate::*;

pub struct IdentifierResolutionCheck;

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for IdentifierResolutionCheck {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        IdentifierResolution::resolve_program(program, ctx);
    }
}

pub struct IdentifierResolution;
