#![allow(dead_code)]

use crate::parse::lexer::Token;
use std::boxed::Box;

// AST
#[derive(Debug, PartialEq)]
pub enum AstTree {
    Concat(Box<AstTree>, Box<AstTree>),
    Or(Box<AstTree>, Box<AstTree>),
    Repeat(Box<AstTree>),
    Literal(char),
    Dot,
    Plus(Box<AstTree>),
    Question(Box<AstTree>),
}

// VMエンジンで使用
#[derive(Debug)]
pub struct Ast<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Ast<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Ast { tokens, index: 0 }
    }

    // トークンをパースし、ASTツリー生成
    pub fn parse(&mut self) -> AstTree {
        self.expr()
    }

    fn expr(&mut self) -> AstTree {
        self.sub_expr()
    }

    // seq '|' seq
    fn sub_expr(&mut self) -> AstTree {
        let a1 = self.seq();
        if self.index >= self.tokens.len() {
            return a1;
        }

        match self.tokens[self.index] {
            Token::Or => {
                self.next();
                let a2 = self.seq();
                AstTree::Or(Box::new(a1), Box::new(a2))
            }
            _ => a1,
        }
    }

    // sub_seq seq
    fn seq(&mut self) -> AstTree {
        let f1 = self.sub_seq();
        if self.index >= self.tokens.len() {
            return f1;
        }

        match self.tokens[self.index] {
            Token::Character(_) | Token::Dot => {
                let f2 = self.seq();
                AstTree::Concat(Box::new(f1), Box::new(f2))
            }
            _ => f1,
        }
    }

    // factor ('*'|'+'|'.'|'?') | factor
    fn sub_seq(&mut self) -> AstTree {
        let f = self.factor();
        if self.index >= self.tokens.len() {
            return f;
        }

        match self.tokens[self.index] {
            Token::Asterisk => {
                self.next();
                AstTree::Repeat(Box::new(f))
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
    fn factor(&mut self) -> AstTree {
        match self.tokens[self.index] {
            Token::Dot => {
                self.next();
                AstTree::Dot
            }
            Token::Character(c) => {
                self.next();
                AstTree::Literal(c)
            }
            _ => self.sub_expr(),
        }
    }

    // プラス演算子作成
    fn plus(&mut self) -> AstTree {
        // 前の文字よりインスタンスを作成
        match self.tokens[self.index - 1] {
            Token::Character(c) => AstTree::Plus(Box::new(AstTree::Literal(c))),
            _ => panic!(
                "[Parser::plus] prev token is not character ({:?}",
                self.tokens[self.index - 1]
            ),
        }
    }

    // ?演算子作成
    fn question(&mut self) -> AstTree {
        // 前の文字よりインスタンスを作成
        match self.tokens[self.index - 1] {
            Token::Character(c) => AstTree::Question(Box::new(AstTree::Literal(c))),
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
    fn test_ast_literal() {
        {
            let tokens = vec![Token::Character('a')];
            let ast = Ast::new(&tokens).parse();

            assert_eq!(AstTree::Literal('a'), ast)
        }
        {
            let tokens = vec![Token::Character('a'), Token::Character('b')];
            let ast = Ast::new(&tokens).parse();

            assert_eq!(
                AstTree::Concat(
                    Box::new(AstTree::Literal('a')),
                    Box::new(AstTree::Literal('b'))
                ),
                ast
            );
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Character('b'),
                Token::Character('c'),
            ];
            let ast = Ast::new(&tokens).parse();

            assert_eq!(
                AstTree::Concat(
                    Box::new(AstTree::Literal('a')),
                    Box::new(AstTree::Concat(
                        Box::new(AstTree::Literal('b')),
                        Box::new(AstTree::Literal('c'))
                    ))
                ),
                ast
            );
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Character('b'),
                Token::Character('c'),
                Token::Character('d'),
                Token::Character('e'),
            ];
            let ast = Ast::new(&tokens).parse();

            assert_eq!(
                AstTree::Concat(
                    Box::new(AstTree::Literal('a')),
                    Box::new(AstTree::Concat(
                        Box::new(AstTree::Literal('b')),
                        Box::new(AstTree::Concat(
                            Box::new(AstTree::Literal('c')),
                            Box::new(AstTree::Concat(
                                Box::new(AstTree::Literal('d')),
                                Box::new(AstTree::Literal('e')),
                            ))
                        ))
                    ))
                ),
                ast
            );
        }
    }

    #[test]
    fn test_ast_asterisk() {
        let tokens = vec![Token::Character('a'), Token::Asterisk];
        let ast = Ast::new(&tokens).parse();

        assert_eq!(AstTree::Repeat(Box::new(AstTree::Literal('a'))), ast)
    }

    #[test]
    fn test_ast_dot() {
        let tokens = vec![Token::Character('a'), Token::Dot, Token::Character('c')];
        let ast = Ast::new(&tokens).parse();

        assert_eq!(
            AstTree::Concat(
                Box::new(AstTree::Literal('a')),
                Box::new(AstTree::Concat(
                    Box::new(AstTree::Dot),
                    Box::new(AstTree::Literal('c')),
                ))
            ),
            ast
        )
    }

    #[test]
    fn test_ast_or() {
        {
            let tokens = vec![Token::Character('a'), Token::Or, Token::Character('b')];
            let ast = Ast::new(&tokens).parse();
            assert_eq!(
                AstTree::Or(
                    Box::new(AstTree::Literal('a')),
                    Box::new(AstTree::Literal('b')),
                ),
                ast
            )
        }
        {
            let tokens = vec![
                Token::Character('a'),
                Token::Or,
                Token::Character('b'),
                Token::Dot,
            ];
            let ast = Ast::new(&tokens).parse();
            assert_eq!(
                AstTree::Or(
                    Box::new(AstTree::Literal('a')),
                    Box::new(AstTree::Concat(
                        Box::new(AstTree::Literal('b')),
                        Box::new(AstTree::Dot),
                    ))
                ),
                ast
            )
        }
    }

    #[test]
    fn test_ast_question() {
        let tokens = vec![Token::Character('a'), Token::Question];
        let ast = Ast::new(&tokens).parse();

        assert_eq!(AstTree::Question(Box::new(AstTree::Literal('a'))), ast)
    }
}
