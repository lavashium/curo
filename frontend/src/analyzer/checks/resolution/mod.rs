mod program;
mod declaration;
mod function_declaration;
mod block;
mod block_item;
mod statement;
mod variable_declaration;
mod expression;

use std::marker::PhantomData;

use common::*;
use language::*;
use crate::*;

pub struct IdentifierResolutionCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for IdentifierResolutionCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}


impl Factory<(), TypedProgram> for IdentifierResolutionCheck<'_, '_> {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        IdentifierResolution::run(program, ctx);
    }
}

pub struct IdentifierResolution<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for IdentifierResolution<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}
