pub mod base;
pub mod concat;
pub mod dot;
pub mod empty;
pub mod literal;
pub mod or;
pub mod plus;
pub mod repeat;

#[cfg(test)]
mod test {
    use crate::automaton::pattern::{
        base::BasePattern, concat::Concat, empty::Empty, literal::Literal, or::Or,
    };
    use std::boxed::Box;

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
