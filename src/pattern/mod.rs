mod base;
mod concat;
mod dot;
mod empty;
mod literal;
mod or;
mod repeat;

#[cfg(test)]
mod test {
    use crate::pattern::{base::BasePattern, concat::Concat, literal::Literal, repeat::Repeat};

    // 正規表現「a+」
    #[test]
    fn test_plus_regex() {
        let a1 = Literal::new('a');
        let a2 = Literal::new('a');
        let r = Repeat::new(&a2);
        let con = Concat::new(&a1, &r);

        assert_eq!(true, con.is_match("a"));
        assert_eq!(true, con.is_match("aa"));
        assert_eq!(true, con.is_match("aaaaaaa"));
        assert_eq!(false, con.is_match(""));
        assert_eq!(false, con.is_match("b"));
        assert_eq!(false, con.is_match("bb"));
    }
}
