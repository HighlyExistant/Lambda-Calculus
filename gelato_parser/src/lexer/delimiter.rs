use std::fmt::{Display, Write};

use crate::lexer::{Lexer, Parse, span::Span, tokens::Tokens};


#[derive(Debug, Clone)]
pub struct Delimiter {
    pub span: Span,
    pub open: char,
    pub close: char,
    pub tokens: Tokens,
}
impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.open).as_str())?;
        let first_span = self.tokens.tokens[0].span();
        let mut prev_span = first_span.range.start..first_span.range.start;
        for token in self.tokens.clone() {
            let current_span = token.span().range.clone();

            for _ in 0..(current_span.start-prev_span.end) {
                f.write_char(' ')?;
            }
            f.write_str(format!("{}", token).as_str())?;
            prev_span = current_span;
        }
        f.write_str(format!("{}", self.close).as_str())
    }
}
impl Parse for Delimiter {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let start = tokenizer.cursor;
        let open = tokenizer.next_char();
        let mut del = Delimiter {
            span: Span::new(0..0),
            open,
            close: '\0',
            tokens: Tokens { tokens: vec![], next: 0 }
        };
        while let Some(token) = tokenizer.parse_token() {
            tokenizer.skip_whitespace();
            let get = tokenizer.get_char();
            del.tokens.tokens.push(token);
            match open {
                '(' => if get == ')' { 
                    del.close = get;
                    break; 
                }
                '{' => if get == '}' { 
                    del.close = get;
                    break; 
                }
                '<' => if get == '>' { 
                    del.close = get;
                    break; 
                }
                '[' => if get == ']' { 
                    del.close = get;
                    break; 
                }
                _ => {}
            }
        }
        tokenizer.cursor += 1;
        let end = tokenizer.cursor;
        del.span.range = start..end;
        Some(del)
    }
}