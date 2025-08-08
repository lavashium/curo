mod program;
mod top_level;
mod instruction;

use std::marker::PhantomData;
use crate::*;
use common::*;

pub struct LegalizerLegalizations<'scp, 'ctx> {
    _driver: PhantomData<LegalizerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for LegalizerLegalizations<'scp, 'ctx> {
    type Context = LegalizerContext<'scp, 'ctx>;
}
