#![allow(dead_code)]

// サポートしているトークン
#[derive(Debug, PartialEq)]
pub enum Token {
    Asterisk,
    Dot,
    Character(char),
    Or,
    Plus,
    Question,
}

pub struct Lexer<'a> {
    str: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Lexer { str }
    }

    // 与えられた文字列を解析し、トークン列を返す
    pub fn scan(&self) -> Vec<Token> {
        self.str
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| match c {
                s if s.is_alphanumeric() => Token::Character(c),
                '*' => Token::Asterisk,
                '.' => Token::Dot,
                '+' => Token::Plus,
                '|' => Token::Or,
                '?' => Token::Question,
                _ => panic!("[Lexer::scan] not support char: {:?}", c),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scan_character() {
        let tokens = Lexer::new("abc").scan();

        assert_eq!(3, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Character('b'), tokens[1]);
        assert_eq!(Token::Character('c'), tokens[2]);
    }

    #[test]
    fn test_scan_character_include_newline() {
        let tokens = Lexer::new("abc\nd").scan();

        assert_eq!(4, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Character('b'), tokens[1]);
        assert_eq!(Token::Character('c'), tokens[2]);
        assert_eq!(Token::Character('d'), tokens[3]);
    }

    #[test]
    fn test_scan_number() {
        let tokens = Lexer::new("a1c0").scan();

        assert_eq!(4, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Character('1'), tokens[1]);
        assert_eq!(Token::Character('c'), tokens[2]);
        assert_eq!(Token::Character('0'), tokens[3]);
    }

    #[test]
    fn test_scan_asterisk() {
        let tokens = Lexer::new("ab*").scan();

        assert_eq!(3, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Character('b'), tokens[1]);
        assert_eq!(Token::Asterisk, tokens[2]);
    }

    #[test]
    fn test_scan_plus() {
        let tokens = Lexer::new("ab+").scan();

        assert_eq!(3, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Character('b'), tokens[1]);
        assert_eq!(Token::Plus, tokens[2]);
    }

    #[test]
    fn test_scan_dot() {
        let tokens = Lexer::new("a.b").scan();

        assert_eq!(3, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Dot, tokens[1]);
        assert_eq!(Token::Character('b'), tokens[2]);
    }

    #[test]
    fn test_scan_or() {
        let tokens = Lexer::new("a|b").scan();

        assert_eq!(3, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Or, tokens[1]);
        assert_eq!(Token::Character('b'), tokens[2]);
    }

    #[test]
    fn test_scan_question() {
        let tokens = Lexer::new("a?").scan();

        assert_eq!(2, tokens.len());
        assert_eq!(Token::Character('a'), tokens[0]);
        assert_eq!(Token::Question, tokens[1]);
    }
}
