mod program;
mod function_declaration;
mod block;
mod block_item;
mod statement;

use common::*;
use language::*;
use crate::*;

pub struct LoopLabelingCheck;

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for LoopLabelingCheck {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext<'_, '_>) {
        LoopLabeling::label_program(program, ctx)
    }
}

pub struct LoopLabeling;