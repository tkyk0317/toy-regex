pub mod base;
pub mod concat;
pub mod dot;
pub mod empty;
pub mod literal;
pub mod or;
pub mod repeat;

#[cfg(test)]
mod test {
    use crate::automaton::pattern::{
        base::BasePattern, concat::Concat, empty::Empty, literal::Literal, or::Or, repeat::Repeat,
    };
    use std::boxed::Box;

    // 正規表現「a+」
    #[test]
    fn test_plus_regex() {
        let a1 = Literal::new('a');
        let a2 = Literal::new('a');
        let r = Repeat::new(&a2);
        let con = Concat::new(Box::new(a1), Box::new(r));

        assert_eq!(true, con.is_match("a"));
        assert_eq!(true, con.is_match("aa"));
        assert_eq!(true, con.is_match("aaaaaaa"));
        assert_eq!(false, con.is_match(""));
        assert_eq!(false, con.is_match("b"));
        assert_eq!(false, con.is_match("bb"));
    }

    // 正規表現「?」
    #[test]
    fn test_question_regex() {
        // a?のテスト
        {
            let a = Literal::new('a');
            let e = Empty::new();
            let or = Or::new(Box::new(a), Box::new(e));

            assert_eq!(true, or.is_match("a"));
            assert_eq!(true, or.is_match(""));
            assert_eq!(false, or.is_match("aa"));
        }
        // a?bのテスト
        {
            let a = Literal::new('a');
            let b1 = Literal::new('b');
            let b2 = Literal::new('b');
            let c = Concat::new(Box::new(a), Box::new(b1));
            let or = Or::new(Box::new(c), Box::new(b2));

            assert_eq!(true, or.is_match("ab"));
            assert_eq!(true, or.is_match("b"));
            assert_eq!(false, or.is_match("aab"));
            assert_eq!(false, or.is_match(""));
        }
    }
}
