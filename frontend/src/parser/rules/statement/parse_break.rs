use crate::parser::rules::*;
use language::*;

pub fn parse_break(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    parser.expect(token_keyword!(Break))?;
    parser.expect(token_punctuation!(Semicolon))?;
    Some(AstStatement::new_break(
        String::new(),
    ))
}