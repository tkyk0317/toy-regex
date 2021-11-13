#![allow(dead_code)]

use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{concat::Concat, literal::Literal};
use crate::parser::lexer::Token;
use std::boxed::Box;

struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, index: 0 }
    }

    // トークン列をパース
    pub fn parse(&mut self) -> Box<dyn BasePattern> {
        self.expr()
    }

    fn expr(&mut self) -> Box<dyn BasePattern> {
        self.sub_expr()
    }

    // seq ('*'|'+') | factor
    fn sub_expr(&mut self) -> Box<dyn BasePattern> {
        self.seq()
    }

    // seq factor | factor
    fn seq(&mut self) -> Box<dyn BasePattern> {
        let f1 = self.factor();

        if self.index < self.tokens.len() {
            match self.tokens[self.index] {
                Token::Character(_) => {
                    let f2 = self.seq();
                    Box::new(Concat::new(f1, f2))
                }
                _ => f1,
            }
        } else {
            f1
        }
    }

    // Literal
    fn factor(&mut self) -> Box<dyn BasePattern> {
        let factor = match self.tokens[self.index] {
            Token::Character(c) => Literal::new(c),
            _ => panic!(
                "[Parser::factor] not support token ({:?})",
                self.tokens[self.index]
            ),
        };

        self.index += 1;

        Box::new(factor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_literal() {
        {
            let tokens = vec![Token::Character('a')];
            let p = Parser::new(&tokens).parse();

            assert_eq!(true, p.is_match("a"));
            assert_eq!(false, p.is_match("b"));
        }
        {
            let tokens = vec![Token::Character('a'), Token::Character('b')];
            let p = Parser::new(&tokens).parse();

            assert_eq!(true, p.is_match("ab"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match("b"));
            assert_eq!(false, p.is_match("aa"));
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Character('b'),
                Token::Character('c'),
                Token::Character('d'),
                Token::Character('e'),
            ];
            let p = Parser::new(&tokens).parse();

            assert_eq!(true, p.is_match("abcde"));
            assert_eq!(false, p.is_match("abcdef"));
            assert_eq!(false, p.is_match("abcd"));
        }
    }
}
