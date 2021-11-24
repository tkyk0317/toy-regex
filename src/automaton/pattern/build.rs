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
            AstTree::Plus(ast) => Box::new(Plus::new(self.to_pattern(ast), self.to_pattern(ast))),
            AstTree::Question(ast) => Box::new(Question::new(self.to_pattern(ast))),
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
            assert!(Builder::new("a").to_dfa().is_match("a"));
            assert!(!Builder::new("a").to_dfa().is_match("b"));
        }
        {
            assert!(Builder::new("ab").to_dfa().is_match("ab"));
            assert!(!Builder::new("ab").to_dfa().is_match("a"));
            assert!(!Builder::new("ab").to_dfa().is_match("b"));
            assert!(!Builder::new("ab").to_dfa().is_match("aa"));
        }
        {
            assert!(Builder::new("abcde").to_dfa().is_match("abcde"));
            assert!(!Builder::new("abcde").to_dfa().is_match("abcdef"));
            assert!(!Builder::new("abcde").to_dfa().is_match("abcd"));
        }
    }

    #[test]
    fn test_parser_asterisk() {
        {
            assert!(Builder::new("a*").to_dfa().is_match(""));
            assert!(Builder::new("a*").to_dfa().is_match("a"));
            assert!(Builder::new("a*").to_dfa().is_match("aa"));
            assert!(!Builder::new("a*").to_dfa().is_match("ab"));
            assert!(!Builder::new("a*").to_dfa().is_match("b"));
        }
        {
            assert!(Builder::new("b*").to_dfa().is_match(""));
            assert!(Builder::new("b*").to_dfa().is_match("b"));
            assert!(Builder::new("b*").to_dfa().is_match("bbbbbbb"));
            assert!(!Builder::new("b*").to_dfa().is_match("a"));
            assert!(!Builder::new("b*").to_dfa().is_match("ba"));
        }
    }
    #[test]
    fn test_parser_plus() {
        assert!(Builder::new("a+").to_dfa().is_match("a"));
        assert!(Builder::new("a+").to_dfa().is_match("aa"));
        assert!(Builder::new("a+").to_dfa().is_match("aaa"));
        assert!(!Builder::new("a+").to_dfa().is_match("b"));
        assert!(!Builder::new("a+").to_dfa().is_match(""));
    }

    #[test]
    fn test_parser_dot() {
        {
            assert!(Builder::new(".").to_dfa().is_match("a"));
            assert!(!Builder::new(".").to_dfa().is_match("ab"));
            assert!(!Builder::new(".").to_dfa().is_match("aa"));
            assert!(!Builder::new(".").to_dfa().is_match(""));
        }
        {
            assert!(Builder::new("a.").to_dfa().is_match("aa"));
            assert!(Builder::new("a.").to_dfa().is_match("ab"));
            assert!(!Builder::new("a.").to_dfa().is_match("a"));
            assert!(!Builder::new("a.").to_dfa().is_match("abb"));
            assert!(!Builder::new("a.").to_dfa().is_match(""));
        }
        {
            assert!(Builder::new("a..").to_dfa().is_match("aaa"));
            assert!(Builder::new("a..").to_dfa().is_match("abc"));
            assert!(!Builder::new("a..").to_dfa().is_match("aaaa"));
            assert!(!Builder::new("a..").to_dfa().is_match("aa"));
            assert!(!Builder::new("a..").to_dfa().is_match("ab"));
            assert!(!Builder::new("a..").to_dfa().is_match("a"));
            assert!(!Builder::new("a..").to_dfa().is_match("b"));
            assert!(!Builder::new("a..").to_dfa().is_match(""));
        }
        {
            assert!(Builder::new("a.c").to_dfa().is_match("aac"));
            assert!(Builder::new("a.c").to_dfa().is_match("abc"));
            assert!(!Builder::new("a.c").to_dfa().is_match("aaaa"));
            assert!(!Builder::new("a.c").to_dfa().is_match("aa"));
            assert!(!Builder::new("a.c").to_dfa().is_match("ab"));
            assert!(!Builder::new("a.c").to_dfa().is_match("a"));
            assert!(!Builder::new("a.c").to_dfa().is_match("b"));
            assert!(!Builder::new("a.c").to_dfa().is_match(""));
        }
        {
            assert!(Builder::new(".a").to_dfa().is_match("aa"));
            assert!(Builder::new(".a").to_dfa().is_match("ba"));
            assert!(!Builder::new(".a").to_dfa().is_match(""));
            assert!(!Builder::new(".a").to_dfa().is_match("ab"));
            assert!(!Builder::new(".a").to_dfa().is_match("baa"));
            assert!(!Builder::new(".a").to_dfa().is_match("bab"));
        }
    }
    #[test]
    fn test_parser_or() {
        {
            assert!(Builder::new("a|b").to_dfa().is_match("a"));
            assert!(Builder::new("a|b").to_dfa().is_match("b"));
            assert!(!Builder::new("a|b").to_dfa().is_match("c"));
            assert!(!Builder::new("a|b").to_dfa().is_match("ab"));
        }
        {
            assert!(Builder::new("ab|cd").to_dfa().is_match("ab"));
            assert!(Builder::new("ab|cd").to_dfa().is_match("cd"));
            assert!(!Builder::new("ab|cd").to_dfa().is_match("a"));
            assert!(!Builder::new("ab|cd").to_dfa().is_match("b"));
            assert!(!Builder::new("ab|cd").to_dfa().is_match("c"));
            assert!(!Builder::new("ab|cd").to_dfa().is_match("d"));
            assert!(!Builder::new("ab|cd").to_dfa().is_match("abcd"));
        }
    }
    #[test]
    fn test_parser_question() {
        {
            assert!(Builder::new("a?").to_dfa().is_match("a"));
            assert!(Builder::new("a?").to_dfa().is_match(""));
            assert!(!Builder::new("a?").to_dfa().is_match("aa"));
        }
        {
            assert!(Builder::new("a?b").to_dfa().is_match("ab"));
            assert!(Builder::new("a?b").to_dfa().is_match("b"));
            assert!(!Builder::new("a?b").to_dfa().is_match("a"));
            assert!(!Builder::new("a?b").to_dfa().is_match(""));
            assert!(!Builder::new("a?b").to_dfa().is_match("aa"));
        }
        {
            assert!(Builder::new("ab?c").to_dfa().is_match("abc"));
            assert!(Builder::new("ab?c").to_dfa().is_match("ac"));
            assert!(!Builder::new("ab?c").to_dfa().is_match("a"));
            assert!(!Builder::new("ab?c").to_dfa().is_match("b"));
            assert!(!Builder::new("ab?c").to_dfa().is_match(""));
            assert!(!Builder::new("ab?c").to_dfa().is_match("ab"));
        }
        {
            assert!(Builder::new("a?c?").to_dfa().is_match(""));
            assert!(Builder::new("a?c?").to_dfa().is_match("a"));
            assert!(Builder::new("a?c?").to_dfa().is_match("c"));
            assert!(Builder::new("a?c?").to_dfa().is_match("ac"));
            assert!(!Builder::new("a?c?").to_dfa().is_match("b"));
        }
    }
}
