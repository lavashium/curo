mod resolution;

pub use resolution::*;

use language::*;
use super::*;

macro_rules! auto_nest {
    () => {
        ()
    };
    ($head:ty $(, $tail:ty)* $(,)?) => {
        ($head, auto_nest!($($tail),*))
    };
}

pub trait SemanticChecklist {
    fn run_all(ast: &mut AstProgram, ctx: &mut SemanticContext);
}

impl SemanticChecklist for () {
    fn run_all(_: &mut AstProgram, _: &mut SemanticContext) {}
}

impl<Head: SemanticCheck, Tail: SemanticChecklist> SemanticChecklist for (Head, Tail) {
    fn run_all(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        Head::analyze(ast, ctx);
        Tail::run_all(ast, ctx);
    }
}

pub type CHECKS = auto_nest!(
    VariableResolutionCheck,
);

pub trait SemanticCheck {
    fn analyze<'a>(ast: &mut AstProgram, ctx: &'a mut SemanticContext);
}

