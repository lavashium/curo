mod top_level;
mod instruction;
mod program;

use std::marker::PhantomData;
use crate::*;
use common::*;

pub struct GeneratorCasts<'scp, 'ctx> {
    _driver: PhantomData<GeneratorContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for GeneratorCasts<'scp, 'ctx> {
    type Context = GeneratorContext<'scp, 'ctx>;
}
