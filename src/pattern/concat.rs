#![allow(dead_code)]

use crate::farule::FARule;
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

#[derive(Debug)]
struct Concat<'a, T: BasePattern, U: BasePattern> {
    left: &'a T,
    right: &'a U,
}

impl<'a, T: BasePattern, U: BasePattern> Concat<'a, T, U> {
    pub fn new(left: &'a T, right: &'a U) -> Self {
        Concat {
            left: left,
            right: right,
        }
    }
}

impl<'a, T: BasePattern, U: BasePattern> BasePattern for Concat<'a, T, U> {
    fn is_match(&self, s: &str) -> bool {
        let rule = self.rule();
        NFADesign::new(
            rule[0].state,
            &vec![rule.last().unwrap().next_state],
            &NFARulebook::new(rule),
        )
        .accept(s)
    }

    fn rule(&self) -> Vec<FARule> {
        // 左辺と右辺のルールを結合
        let mut l_rule = self.left.rule();
        let r_rule = self.right.rule();

        // ε遷移を挟んで、左の末尾と右の開始をつねげる
        let epsilon = FARule::new(l_rule.last().unwrap().next_state, None, r_rule[0].state);
        l_rule.push(epsilon);
        l_rule.extend(r_rule);
        l_rule
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pattern::empty::Empty;
    use crate::pattern::literal::Literal;

    #[test]
    fn test_concat() {
        {
            let l = Literal::new('a');
            let r = Literal::new('b');
            let c = Concat::new(&l, &r);

            assert_eq!(true, c.is_match("ab"));
            assert_eq!(false, c.is_match("aa"));
        }
        {
            let a = Literal::new('a');
            let b = Literal::new('b');
            let c = Literal::new('c');
            let c2 = Concat::new(&b, &c);
            let c1 = Concat::new(&a, &c2);

            assert_eq!(true, c1.is_match("abc"));
            assert_eq!(false, c1.is_match("abcc"));
        }
        {
            let a = Literal::new('a');
            let b = Literal::new('b');
            let c = Literal::new('c');
            let c1 = Concat::new(&a, &b);
            let c2 = Concat::new(&c1, &c);

            assert_eq!(true, c2.is_match("abc"));
            assert_eq!(false, c2.is_match("abcc"));
        }
        {
            let l = Literal::new('a');
            let r = Empty::new();
            let c = Concat::new(&l, &r);

            assert_eq!(true, c.is_match("a\0"));
            assert_eq!(false, c.is_match("ab"));
        }
    }
}
