use super::*;
use language::*;

pub trait ProgramParser {
    fn parse_program(&mut self) -> ParseResult<AstProgram>;
}

impl<'a> ProgramParser for ParserRules<'a> {
    fn parse_program(&mut self) -> ParseResult<AstProgram> {
        let mut functions = Vec::new();
        while self.parser.source_tokens.peek()?.kind() != &TokenKind::EOF {
            functions.push(self.parse_function_declaration()?);
        }
        self.expect(TokenKind::EOF)?;
        Some(AstProgram::new(functions))
    }
}