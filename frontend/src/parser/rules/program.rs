use language::*;
use super::*;

pub trait ProgramParser<'a> {
    fn parse_program(&mut self) -> ParseResult<AstProgram>;
}

impl<'a> ProgramParser<'a> for ParserRules<'a> {
    fn parse_program(&mut self) -> ParseResult<AstProgram> {
        let function = self.parse_function()?;

        self.expect(token_eof!());

        Some(AstProgram {
            function_definition: function,
        })
    }
}