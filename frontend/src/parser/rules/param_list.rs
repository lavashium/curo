use super::*;
use language::*;

pub trait ParamListParser {
    fn parse_param_list(&mut self) -> ParseResult<Vec<String>>;
}

impl<'a> ParamListParser for ParserRules<'a> {
    fn parse_param_list(&mut self) -> ParseResult<Vec<String>> {
        let mut params = Vec::new();
        
        if self.parser.source_tokens.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Void) {
            self.parser.source_tokens.consume()?;
            return Some(params);
        }
        
        loop {
            self.expect(TokenKind::Keyword(KeywordKind::Int))?;
            let id = self.unwrap_identifier()?;
            params.push(id);
            
            if self.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::Comma) {
                break;
            }
            self.parser.source_tokens.consume()?;
        }
        
        Some(params)
    }
}
