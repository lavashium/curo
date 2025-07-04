use super::*;
use language::*;

pub trait ProgramParser {
    fn parse_program(&mut self) -> ParseResult<AstProgram>;
}

impl<'a> ProgramParser for ParserRules<'a> {
    fn parse_program(&mut self) -> ParseResult<AstProgram> {
        let function = self.parse_function()?;

        self.expect(token_eof!());

        Some(AstProgram {
            function_definition: function,
        })
    }
}
