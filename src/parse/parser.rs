#![allow(dead_code)]

use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{
    concat::Concat, dot::Dot, literal::Literal, or::Or, plus::Plus, question::Question,
    repeat::Repeat,
};
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

    // seq '|' seq
    fn sub_expr(&mut self) -> Box<dyn BasePattern> {
        let f1 = self.seq();
        if self.index >= self.tokens.len() {
            return f1;
        }

        match self.tokens[self.index] {
            Token::Or => {
                self.next();
                let f2 = self.seq();
                Box::new(Or::new(f1, f2))
            }
            _ => f1,
        }
    }

    // sub_seq seq
    fn seq(&mut self) -> Box<dyn BasePattern> {
        let f1 = self.sub_seq();
        if self.index >= self.tokens.len() {
            return f1;
        }

        match self.tokens[self.index] {
            Token::Character(_) | Token::Dot => {
                let f2 = self.seq();
                Box::new(Concat::new(f1, f2))
            }
            _ => f1,
        }
    }

    // factor ('*'|'+'|'.'|'?') | factor
    fn sub_seq(&mut self) -> Box<dyn BasePattern> {
        let f = self.factor();
        if self.index >= self.tokens.len() {
            return f;
        }

        match self.tokens[self.index] {
            Token::Asterisk => {
                self.next();
                Box::new(Repeat::new(f))
            }
            Token::Question => {
                let q = self.question();
                self.next();
                q
            }
            Token::Plus => {
                // プラス演算子の分を読み取って、インスタンスを返す
                let f = self.plus();
                self.next();
                f
            }
            _ => f,
        }
    }

    // Literal | '.' | sub_expr
    fn factor(&mut self) -> Box<dyn BasePattern> {
        match self.tokens[self.index] {
            Token::Dot => {
                self.next();
                Box::new(Dot::new())
            }
            Token::Character(c) => {
                self.next();
                Box::new(Literal::new(c))
            }
            _ => self.sub_expr(),
        }
    }

    // プラス演算子作成
    fn plus(&mut self) -> Box<dyn BasePattern> {
        // 前の文字よりインスタンスを作成
        match self.tokens[self.index - 1] {
            Token::Character(c) => Box::new(Plus::new(c)),
            _ => panic!(
                "[Parser::plus] prev token is not character ({:?}",
                self.tokens[self.index - 1]
            ),
        }
    }
    // ?演算子作成
    fn question(&mut self) -> Box<dyn BasePattern> {
        // 前の文字よりインスタンスを作成
        match self.tokens[self.index - 1] {
            Token::Character(c) => Box::new(Question::new(c)),
            _ => panic!(
                "[Parser::question] prev token is not character ({:?}",
                self.tokens[self.index - 1]
            ),
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

    #[test]
    fn test_parser_plus() {
        let tokens = vec![Token::Character('a'), Token::Plus];
        let p = Parser::new(&tokens).parse();

        assert_eq!(true, p.is_match("a"));
        assert_eq!(true, p.is_match("aa"));
        assert_eq!(true, p.is_match("aaa"));
        assert_eq!(false, p.is_match("b"));
        assert_eq!(false, p.is_match(""));
    }
    #[test]
    fn test_parser_dot() {
        {
            let tokens = vec![Token::Dot];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("a"));
            assert_eq!(false, p.is_match("ab"));
            assert_eq!(false, p.is_match("aa"));
            assert_eq!(false, p.is_match(""));
        }
        {
            let tokens = vec![Token::Character('a'), Token::Dot];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("aa"));
            assert_eq!(true, p.is_match("ab"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match("abb"));
            assert_eq!(false, p.is_match(""));
        }
        {
            let tokens = vec![Token::Character('a'), Token::Dot, Token::Dot];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("aaa"));
            assert_eq!(true, p.is_match("abc"));
            assert_eq!(false, p.is_match("aaaa"));
            assert_eq!(false, p.is_match("aa"));
            assert_eq!(false, p.is_match("ab"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match("b"));
            assert_eq!(false, p.is_match(""));
        }
        {
            let tokens = vec![Token::Character('a'), Token::Dot, Token::Character('c')];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("aac"));
            assert_eq!(true, p.is_match("abc"));
            assert_eq!(false, p.is_match("aaaa"));
            assert_eq!(false, p.is_match("aa"));
            assert_eq!(false, p.is_match("ab"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match("b"));
            assert_eq!(false, p.is_match(""));
        }
        {
            let tokens = vec![Token::Dot, Token::Character('a')];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("aa"));
            assert_eq!(true, p.is_match("ba"));
            assert_eq!(false, p.is_match("ab"));
            assert_eq!(false, p.is_match("baa"));
            assert_eq!(false, p.is_match("bab"));
            assert_eq!(false, p.is_match(""));
        }
    }

    #[test]
    fn test_parser_or() {
        {
            let tokens = vec![Token::Character('a'), Token::Or, Token::Character('b')];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("a"));
            assert_eq!(true, p.is_match("b"));
            assert_eq!(false, p.is_match("c"));
            assert_eq!(false, p.is_match("ab"));
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Character('b'),
                Token::Or,
                Token::Character('c'),
                Token::Character('d'),
            ];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("ab"));
            assert_eq!(true, p.is_match("cd"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match("b"));
            assert_eq!(false, p.is_match("c"));
            assert_eq!(false, p.is_match("d"));
            assert_eq!(false, p.is_match("abcd"));
        }
    }

    #[test]
    fn test_parser_question() {
        {
            let tokens = vec![Token::Character('a'), Token::Question];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("a"));
            assert_eq!(true, p.is_match(""));
            assert_eq!(false, p.is_match("aa"));
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Question,
                Token::Character('b'),
            ];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("ab"));
            assert_eq!(true, p.is_match("b"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match(""));
            assert_eq!(false, p.is_match("aa"));
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Character('b'),
                Token::Question,
                Token::Character('c'),
            ];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match("abc"));
            assert_eq!(true, p.is_match("ac"));
            assert_eq!(false, p.is_match("a"));
            assert_eq!(false, p.is_match("b"));
            assert_eq!(false, p.is_match(""));
            assert_eq!(false, p.is_match("ab"));
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Question,
                Token::Character('c'),
                Token::Question,
            ];
            let p = Parser::new(&tokens).parse();
            assert_eq!(true, p.is_match(""));
            assert_eq!(true, p.is_match("a"));
            assert_eq!(true, p.is_match("c"));
            assert_eq!(true, p.is_match("ac"));
            assert_eq!(false, p.is_match("b"));
        }
    }
}
