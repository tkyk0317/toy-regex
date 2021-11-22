#![allow(dead_code)]

use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{
    concat::Concat, dot::Dot, literal::Literal, or::Or, plus::Plus, question::Question,
    repeat::Repeat,
};
use crate::parse::lexer::Lexer;
use crate::parse::parser::{Ast, AstTree};

pub struct Builder {
    ast: AstTree,
}

impl Builder {
    pub fn new(pattern: &str) -> Self {
        Builder {
            ast: Ast::new(&Lexer::new(pattern).scan()).parse(),
        }
    }

    // DFAインスタンスを返す
    pub fn to_dfa(&self) -> Box<dyn BasePattern> {
        self.to_pattern(&self.ast)
    }

    // DFA型インスタンスを生成
    fn to_pattern(&self, ast: &AstTree) -> Box<dyn BasePattern> {
        match ast {
            AstTree::Concat(l_ast, r_ast) => {
                let l = self.to_pattern(l_ast);
                let r = self.to_pattern(r_ast);
                Box::new(Concat::new(l, r))
            }
            AstTree::Or(l_ast, r_ast) => {
                let l = self.to_pattern(l_ast);
                let r = self.to_pattern(r_ast);
                Box::new(Or::new(l, r))
            }
            AstTree::Repeat(ast) => {
                let pattern = self.to_pattern(ast);
                Box::new(Repeat::new(pattern))
            }
            AstTree::Literal(c) => Box::new(Literal::new(*c)),
            AstTree::Plus(ast) => match **ast {
                AstTree::Literal(c) => Box::new(Plus::new(c)),
                _ => panic!("[Builder::to_pattern] not support ast in plus ({:?})", ast),
            },
            AstTree::Question(ast) => match **ast {
                AstTree::Literal(c) => Box::new(Question::new(c)),
                _ => panic!(
                    "[Builder::to_pattern] not support ast in question ({:?})",
                    ast
                ),
            },
            AstTree::Dot => Box::new(Dot::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_literal() {
        {
            assert_eq!(true, Builder::new("a").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("a").to_dfa().is_match("b"));
        }
        {
            assert_eq!(true, Builder::new("ab").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new("ab").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("ab").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("ab").to_dfa().is_match("aa"));
        }
        {
            assert_eq!(true, Builder::new("abcde").to_dfa().is_match("abcde"));
            assert_eq!(false, Builder::new("abcde").to_dfa().is_match("abcdef"));
            assert_eq!(false, Builder::new("abcde").to_dfa().is_match("abcd"));
        }
    }

    #[test]
    fn test_parser_asterisk() {
        {
            assert_eq!(true, Builder::new("a*").to_dfa().is_match(""));
            assert_eq!(true, Builder::new("a*").to_dfa().is_match("a"));
            assert_eq!(true, Builder::new("a*").to_dfa().is_match("aa"));
            assert_eq!(false, Builder::new("a*").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new("a*").to_dfa().is_match("b"));
        }
        {
            assert_eq!(true, Builder::new("b*").to_dfa().is_match(""));
            assert_eq!(true, Builder::new("b*").to_dfa().is_match("b"));
            assert_eq!(true, Builder::new("b*").to_dfa().is_match("bbbbbbb"));
            assert_eq!(false, Builder::new("b*").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("b*").to_dfa().is_match("ba"));
        }
    }
    #[test]
    fn test_parser_plus() {
        assert_eq!(true, Builder::new("a+").to_dfa().is_match("a"));
        assert_eq!(true, Builder::new("a+").to_dfa().is_match("aa"));
        assert_eq!(true, Builder::new("a+").to_dfa().is_match("aaa"));
        assert_eq!(false, Builder::new("a+").to_dfa().is_match("b"));
        assert_eq!(false, Builder::new("a+").to_dfa().is_match(""));
    }

    #[test]
    fn test_parser_dot() {
        {
            assert_eq!(true, Builder::new(".").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new(".").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new(".").to_dfa().is_match("aa"));
            assert_eq!(false, Builder::new(".").to_dfa().is_match(""));
        }
        {
            assert_eq!(true, Builder::new("a.").to_dfa().is_match("aa"));
            assert_eq!(true, Builder::new("a.").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new("a.").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("a.").to_dfa().is_match("abb"));
            assert_eq!(false, Builder::new("a.").to_dfa().is_match(""));
        }
        {
            assert_eq!(true, Builder::new("a..").to_dfa().is_match("aaa"));
            assert_eq!(true, Builder::new("a..").to_dfa().is_match("abc"));
            assert_eq!(false, Builder::new("a..").to_dfa().is_match("aaaa"));
            assert_eq!(false, Builder::new("a..").to_dfa().is_match("aa"));
            assert_eq!(false, Builder::new("a..").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new("a..").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("a..").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("a..").to_dfa().is_match(""));
        }
        {
            assert_eq!(true, Builder::new("a.c").to_dfa().is_match("aac"));
            assert_eq!(true, Builder::new("a.c").to_dfa().is_match("abc"));
            assert_eq!(false, Builder::new("a.c").to_dfa().is_match("aaaa"));
            assert_eq!(false, Builder::new("a.c").to_dfa().is_match("aa"));
            assert_eq!(false, Builder::new("a.c").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new("a.c").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("a.c").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("a.c").to_dfa().is_match(""));
        }
        {
            assert_eq!(true, Builder::new(".a").to_dfa().is_match("aa"));
            assert_eq!(true, Builder::new(".a").to_dfa().is_match("ba"));
            assert_eq!(false, Builder::new(".a").to_dfa().is_match(""));
            assert_eq!(false, Builder::new(".a").to_dfa().is_match("ab"));
            assert_eq!(false, Builder::new(".a").to_dfa().is_match("baa"));
            assert_eq!(false, Builder::new(".a").to_dfa().is_match("bab"));
        }
    }
    #[test]
    fn test_parser_or() {
        {
            assert_eq!(true, Builder::new("a|b").to_dfa().is_match("a"));
            assert_eq!(true, Builder::new("a|b").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("a|b").to_dfa().is_match("c"));
            assert_eq!(false, Builder::new("a|b").to_dfa().is_match("ab"));
        }
        {
            assert_eq!(true, Builder::new("ab|cd").to_dfa().is_match("ab"));
            assert_eq!(true, Builder::new("ab|cd").to_dfa().is_match("cd"));
            assert_eq!(false, Builder::new("ab|cd").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("ab|cd").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("ab|cd").to_dfa().is_match("c"));
            assert_eq!(false, Builder::new("ab|cd").to_dfa().is_match("d"));
            assert_eq!(false, Builder::new("ab|cd").to_dfa().is_match("abcd"));
        }
    }
    #[test]
    fn test_parser_question() {
        {
            assert_eq!(true, Builder::new("a?").to_dfa().is_match("a"));
            assert_eq!(true, Builder::new("a?").to_dfa().is_match(""));
            assert_eq!(false, Builder::new("a?").to_dfa().is_match("aa"));
        }
        {
            assert_eq!(true, Builder::new("a?b").to_dfa().is_match("ab"));
            assert_eq!(true, Builder::new("a?b").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("a?b").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("a?b").to_dfa().is_match(""));
            assert_eq!(false, Builder::new("a?b").to_dfa().is_match("aa"));
        }
        {
            assert_eq!(true, Builder::new("ab?c").to_dfa().is_match("abc"));
            assert_eq!(true, Builder::new("ab?c").to_dfa().is_match("ac"));
            assert_eq!(false, Builder::new("ab?c").to_dfa().is_match("a"));
            assert_eq!(false, Builder::new("ab?c").to_dfa().is_match("b"));
            assert_eq!(false, Builder::new("ab?c").to_dfa().is_match(""));
            assert_eq!(false, Builder::new("ab?c").to_dfa().is_match("ab"));
        }
        {
            assert_eq!(true, Builder::new("a?c?").to_dfa().is_match(""));
            assert_eq!(true, Builder::new("a?c?").to_dfa().is_match("a"));
            assert_eq!(true, Builder::new("a?c?").to_dfa().is_match("c"));
            assert_eq!(true, Builder::new("a?c?").to_dfa().is_match("ac"));
            assert_eq!(false, Builder::new("a?c?").to_dfa().is_match("b"));
        }
    }
}
