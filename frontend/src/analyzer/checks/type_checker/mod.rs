mod program;
mod function_declaration;
mod variable_declaration;
mod block;
mod block_item;
mod declaration;
mod statement;
mod expression;

use std::marker::PhantomData;

use common::*;
use language::*;
use crate::*;

pub struct TypeCheckCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for TypeCheckCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl Factory<(), TypedProgram> for TypeCheckCheck<'_, '_> {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        TypeCheck::run(program, ctx);
    }
}

pub struct TypeCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for TypeCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}
