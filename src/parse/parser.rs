#![allow(dead_code)]

use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{concat::Concat, literal::Literal, repeat::Repeat};
use crate::parse::lexer::Token;
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

    // seq
    fn sub_expr(&mut self) -> Box<dyn BasePattern> {
        self.seq()
    }

    // sub_seq seq
    fn seq(&mut self) -> Box<dyn BasePattern> {
        let f1 = self.sub_seq();
        if self.index >= self.tokens.len() {
            return f1;
        }

        match self.tokens[self.index] {
            Token::Character(_) => {
                let f2 = self.seq();
                Box::new(Concat::new(f1, f2))
            }
            _ => f1,
        }
    }

    // factor ('*'|'+') | factor
    fn sub_seq(&mut self) -> Box<dyn BasePattern> {
        let f1 = self.factor();
        if self.index >= self.tokens.len() {
            return f1;
        }

        match self.tokens[self.index] {
            Token::Asterisk => {
                self.next();
                Box::new(Repeat::new(f1))
            }
            _ => f1,
        }
    }

    // Literal | sub_expr
    fn factor(&mut self) -> Box<dyn BasePattern> {
        match self.tokens[self.index] {
            Token::Character(c) => {
                self.next();
                Box::new(Literal::new(c))
            }
            _ => self.sub_expr(),
        }
    }

    // 次の要素へ移動
    fn next(&mut self) {
        self.index += 1;
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

    #[test]
    fn test_parser_asterisk() {
        {
            let tokens = vec![Token::Character('a'), Token::Asterisk];
            let p = Parser::new(&tokens).parse();

            assert_eq!(true, p.is_match(""));
            assert_eq!(true, p.is_match("a"));
            assert_eq!(true, p.is_match("aa"));
            assert_eq!(false, p.is_match("ab"));
            assert_eq!(false, p.is_match("b"));
        }
        {
            let tokens = vec![Token::Character('b'), Token::Asterisk];
            let p = Parser::new(&tokens).parse();

            assert_eq!(true, p.is_match(""));
            assert_eq!(true, p.is_match("b"));
            assert_eq!(true, p.is_match("bbbbbbb"));
            assert_eq!(false, p.is_match("ba"));
            assert_eq!(false, p.is_match("a"));
        }
    }
}
