mod expression;
mod function_declaration;
mod variable_declaration;
mod block_item;
mod declaration;
mod program;
mod statement;
mod block;
mod for_init;

use std::marker::PhantomData;
use common::*;
use crate::*;

pub struct GeneratorTransforms<'scp, 'ctx> {
    _driver: PhantomData<TacGenContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for GeneratorTransforms<'scp, 'ctx> {
    type Context = TacGenContext<'scp, 'ctx>;
}
