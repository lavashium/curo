mod program;
mod function_declaration;
mod block;
mod block_item;
mod statement;

use program::*;
use function_declaration::*;
use block::*;
use block_item::*;
use statement::*;

use language::*;
use super::*;

pub struct LoopLabelingCheck;

impl SemanticCheck for LoopLabelingCheck {
    fn analyze(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        label_program(ast, ctx, &None);
    }
}
