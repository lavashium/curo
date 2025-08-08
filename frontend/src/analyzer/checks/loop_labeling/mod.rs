mod program;
mod declaration;
mod function_declaration;
mod block;
mod block_item;
mod statement;

use std::marker::PhantomData;

use common::*;
use language::*;
use crate::*;

pub struct LoopLabelingCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for LoopLabelingCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl Factory<(), TypedProgram> for LoopLabelingCheck<'_, '_> {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext<'_, '_>) {
        LoopLabeling::run(program, ctx)
    }
}

pub struct LoopLabeling<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for LoopLabeling<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}
