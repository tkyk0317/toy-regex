#![allow(dead_code)]

use crate::parse::lexer::{Lexer, Token};
use crate::parse::parser::{Ast, AstTree};

// 中間言語
#[derive(Debug, PartialEq, Clone)]
pub enum RegexIR {
    Char(char),
    Split(usize, usize),
    Jmp(usize),
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
        let ast = Ast::new(&self.tokens).parse();
        let mut inst = self.ast_to_inst(&ast);
        inst.push(RegexIR::Match);
        inst
    }

    // ASTからVMコードへコンパイル
    fn ast_to_inst(&mut self, ast: &AstTree) -> Vec<RegexIR> {
        match ast {
            AstTree::Concat(a, b) => {
                let mut l_inst = self.ast_to_inst(a);
                let r_inst = self.ast_to_inst(b);
                l_inst.extend(r_inst);
                l_inst
            }
            AstTree::Literal(c) => {
                self.pc += 1;
                vec![RegexIR::Char(*c)]
            }
            AstTree::Plus(ast) => {
                // xは今の命令を設定
                let x = self.pc;
                let mut inst = self.ast_to_inst(ast);
                // yは次の命令を設定
                let y = self.pc + 1;
                let ir = RegexIR::Split(x, y);
                self.pc += 1;

                inst.push(ir);
                inst
            }
            AstTree::Question(ast) => {
                // xはast命令に設定
                let x = self.pc + 1;
                let ast_inst = self.ast_to_inst(ast);

                // yはast命令の後に設定
                let y = self.pc + 1;

                // 各命令をマージ
                let mut inst = vec![RegexIR::Split(x, y)];
                inst.extend(ast_inst);
                self.pc += 1;
                inst
            }
            AstTree::Repeat(ast) => {
                // xはast命令に設定
                let cur = self.pc;
                let x = self.pc + 1;
                let ast_inst = self.ast_to_inst(ast);

                // yはJmp命令の後に設定
                let y = self.pc + 2;

                // 各命令をマージ
                let mut inst = vec![RegexIR::Split(x, y)];
                inst.extend(ast_inst);
                inst.push(RegexIR::Jmp(cur));
                self.pc += 2;
                inst
            }
            AstTree::Or(left, right) => {
                // xは、leftの命令へ設定
                let x = self.pc + 1;
                let l_inst = self.ast_to_inst(left);
                self.pc += 1; // Jmp命令を差し込むので加算

                // yはright命令に設定
                let y = self.pc + 1;
                let r_inst = self.ast_to_inst(right);

                // IRを構築
                let mut inst = vec![RegexIR::Split(x, y)];
                self.pc += 1;
                inst.extend(l_inst);
                inst.push(RegexIR::Jmp(self.pc));
                inst.extend(r_inst);
                inst
            }
            _ => panic!("[Builder::ast_to_inst] not support ast {:?}", ast),
        }
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

    #[test]
    fn test_builder_compile_repeat() {
        {
            let ir = Builder::new("a*").compile();

            assert_eq!(4, ir.len());
            assert_eq!(RegexIR::Split(1, 3), ir[0]);
            assert_eq!(RegexIR::Char('a'), ir[1]);
            assert_eq!(RegexIR::Jmp(0), ir[2]);
            assert_eq!(RegexIR::Match, ir[3]);
        }
        {
            let ir = Builder::new("aa*").compile();

            assert_eq!(5, ir.len());
            assert_eq!(RegexIR::Char('a'), ir[0]);
            assert_eq!(RegexIR::Split(2, 4), ir[1]);
            assert_eq!(RegexIR::Char('a'), ir[2]);
            assert_eq!(RegexIR::Jmp(1), ir[3]);
            assert_eq!(RegexIR::Match, ir[4]);
        }
        {
            let ir = Builder::new("aa*bb*").compile();

            assert_eq!(9, ir.len());
            assert_eq!(RegexIR::Char('a'), ir[0]);
            assert_eq!(RegexIR::Split(2, 4), ir[1]);
            assert_eq!(RegexIR::Char('a'), ir[2]);
            assert_eq!(RegexIR::Jmp(1), ir[3]);
            assert_eq!(RegexIR::Char('b'), ir[4]);
            assert_eq!(RegexIR::Split(6, 8), ir[5]);
            assert_eq!(RegexIR::Char('b'), ir[6]);
            assert_eq!(RegexIR::Jmp(5), ir[7]);
            assert_eq!(RegexIR::Match, ir[8]);
        }
    }

    #[test]
    fn test_builder_compile_or() {
        {
            let ir = Builder::new("a|b").compile();

            assert_eq!(5, ir.len());
            assert_eq!(RegexIR::Split(1, 3), ir[0]);
            assert_eq!(RegexIR::Char('a'), ir[1]);
            assert_eq!(RegexIR::Jmp(4), ir[2]);
            assert_eq!(RegexIR::Char('b'), ir[3]);
            assert_eq!(RegexIR::Match, ir[4]);
        }
    }

    #[test]
    fn test_builder_compile_question() {
        {
            let ir = Builder::new("a?").compile();

            assert_eq!(3, ir.len());
            assert_eq!(RegexIR::Split(1, 2), ir[0]);
            assert_eq!(RegexIR::Char('a'), ir[1]);
            assert_eq!(RegexIR::Match, ir[2]);
        }
        {
            let ir = Builder::new("a?b?").compile();

            assert_eq!(5, ir.len());
            assert_eq!(RegexIR::Split(1, 2), ir[0]);
            assert_eq!(RegexIR::Char('a'), ir[1]);
            assert_eq!(RegexIR::Split(3, 4), ir[2]);
            assert_eq!(RegexIR::Char('b'), ir[3]);
            assert_eq!(RegexIR::Match, ir[4]);
        }
    }

    #[should_panic]
    #[test]
    fn test_builder_compile_not_support() {
        Builder::new("a.").compile();
    }
}
