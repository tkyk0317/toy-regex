#![allow(dead_code)]

use crate::parse::lexer::{Lexer, Token};

// 中間言語
#[derive(Debug, PartialEq, Clone)]
pub enum RegexIR {
    Char(char),
    Split(usize, usize),
    Match,
}

pub struct Builder {
    pattern: String,
    tokens: Vec<Token>,
    pc: usize,
}

impl Builder {
    pub fn new(pattern: &str) -> Self {
        Builder {
            tokens: Lexer::new(pattern).scan(),
            pattern: pattern.to_string(),
            pc: 0,
        }
    }

    // 中間言語へコンパイル
    pub fn compile(&mut self) -> Vec<RegexIR> {
        let mut ir: Vec<RegexIR> = self
            .tokens
            .clone()
            .iter()
            .map(|t| match t {
                Token::Character(c) => {
                    self.pc += 1;
                    RegexIR::Char(*c)
                }
                Token::Plus => {
                    let ir = RegexIR::Split(self.pc - 1, self.pc + 1);
                    self.pc += 1;
                    ir
                }
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

    #[test]
    fn test_builder_compile_plus() {
        {
            let ir = Builder::new("a+").compile();

            assert_eq!(3, ir.len());
            assert_eq!(RegexIR::Char('a'), ir[0]);
            assert_eq!(RegexIR::Split(0, 2), ir[1]);
            assert_eq!(RegexIR::Match, ir[2]);
        }
        {
            let ir = Builder::new("a+b+").compile();

            assert_eq!(5, ir.len());
            assert_eq!(RegexIR::Char('a'), ir[0]);
            assert_eq!(RegexIR::Split(0, 2), ir[1]);
            assert_eq!(RegexIR::Char('b'), ir[2]);
            assert_eq!(RegexIR::Split(2, 4), ir[3]);
            assert_eq!(RegexIR::Match, ir[4]);
        }
    }

    #[should_panic]
    #[test]
    fn test_builder_compile_not_supprt_token() {
        Builder::new("*").compile();
    }
}
