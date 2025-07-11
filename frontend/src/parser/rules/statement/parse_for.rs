use crate::parser::rules::*;
use language::*;

pub fn parse_for(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(token_keyword!(For))?;
    parser.expect(token_punctuation!(OpenParen))?;
    let for_init = parser.parse_for_init()?;

    let condition = match parser.parser.source_tokens.peek()?.kind() {
        TokenKind::Punctuation(PunctuationKind::Semicolon) => {
            parser.expect(token_punctuation!(Semicolon))?;
            None
        }
        _ => {
            let cond = parser.parse_expression()?;
            parser.expect(token_punctuation!(Semicolon))?;
            Some(cond)
        }
    };

    let post = match parser.parser.source_tokens.peek()?.kind() {
        TokenKind::Punctuation(PunctuationKind::CloseParen) => {
            parser.expect(token_punctuation!(CloseParen))?;
            None
        }
        _ => {
            let post_expr = parser.parse_expression()?;
            parser.expect(token_punctuation!(CloseParen))?;
            Some(post_expr)
        }
    };

    let body = parser.parse_statement()?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);

    Some(AstStatement::new_for(
        for_init,
        condition,
        post,
        Box::new(body),
        String::new(),
        span
    ))
}
