mod expression;
mod function_declaration;
mod variable_declaration;
mod block_item;
mod declaration;
mod program;
mod statement;
mod block;
mod for_init;

use crate::tacgen_ctx::*;
use constructors::constructors;

#[constructors]
pub struct GeneratorTransforms<'scp, 'ctx> {
    ctx: &'scp mut TacGenContext<'scp, 'ctx>
}