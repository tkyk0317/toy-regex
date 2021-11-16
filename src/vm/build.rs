#![allow(dead_code)]

use crate::parse::lexer::{Lexer, Token};

// 中間言語
#[derive(Debug, PartialEq)]
pub enum RegexIR {
    Char(char),
    Split(usize, usize),
    Match,
}

pub struct Builder {
    pattern: String,
    tokens: Vec<Token>,
}

impl Builder {
    pub fn new(pattern: &str) -> Self {
        Builder {
            tokens: Lexer::new(pattern).scan(),
            pattern: pattern.to_string(),
        }
    }

    // 中間言語へコンパイル
    pub fn compile(&self) -> Vec<RegexIR> {
        let mut ir: Vec<RegexIR> = self
            .tokens
            .iter()
            .map(|t| match t {
                Token::Character(c) => RegexIR::Char(*c),
                _ => panic!("[Builder::compile] not support token _{:?}", t),
            })
            .collect();

        ir.push(RegexIR::Match);
        ir
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_builder_compile_only_char() {
        {
            let ir = Builder::new("a").compile();

            assert_eq!(2, ir.len());
            assert_eq!(RegexIR::Char('a'), ir[0]);
            assert_eq!(RegexIR::Match, ir[1]);
        }
        {
            let ir = Builder::new("abcdef").compile();

            assert_eq!(7, ir.len());
            assert_eq!(RegexIR::Char('a'), ir[0]);
            assert_eq!(RegexIR::Char('b'), ir[1]);
            assert_eq!(RegexIR::Char('c'), ir[2]);
            assert_eq!(RegexIR::Char('d'), ir[3]);
            assert_eq!(RegexIR::Char('e'), ir[4]);
            assert_eq!(RegexIR::Char('f'), ir[5]);
            assert_eq!(RegexIR::Match, ir[6]);
        }
    }

    #[should_panic]
    #[test]
    fn test_builder_compile_not_supprt_token() {
        Builder::new("*").compile();
    }
}
