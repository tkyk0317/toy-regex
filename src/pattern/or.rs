#![allow(dead_code)]

use crate::farule::{FARule, State};
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

#[derive(Debug)]
struct Or<'a, T: BasePattern, U: BasePattern> {
    start_state: State,
    left: &'a T,
    right: &'a U,
}

impl<'a, T: BasePattern, U: BasePattern> Or<'a, T, U> {
    pub fn new(left: &'a T, right: &'a U) -> Self {
        Or {
            start_state: State::create_at_rnd(),
            left: left,
            right: right,
        }
    }
}

impl<'a, T: BasePattern, U: BasePattern> BasePattern for Or<'a, T, U> {
    fn is_match(&self, s: &str) -> bool {
        let rules = self.rules();
        NFADesign::new(
            self.start_state,
            &self.accept_state(),
            &NFARulebook::new(rules),
        )
        .accept(s)
    }

    fn rules(&self) -> Vec<FARule> {
        // ε遷移により、二つのルールの開始状態へ遷移
        let ep_to_l_rule = FARule::new(self.start_state, None, self.left.start_state());
        let ep_to_r_rule = FARule::new(self.start_state, None, self.right.start_state());

        // 各ノードのルールを結合
        let mut rules = vec![ep_to_l_rule, ep_to_r_rule];
        let l_rule = self.left.rules();
        let r_rule = self.right.rules();
        rules.extend(l_rule);
        rules.extend(r_rule);
        rules
    }

    fn accept_state(&self) -> Vec<State> {
        let mut l_accept = self.left.accept_state();
        let r_accept = self.right.accept_state();
        l_accept.extend(r_accept);
        l_accept
    }

    fn start_state(&self) -> State {
        self.start_state
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pattern::literal::Literal;

    #[test]
    fn test_or() {
        {
            let l = Literal::new('a');
            let r = Literal::new('b');
            let o = Or::new(&l, &r);

            assert_eq!(true, o.is_match("a"));
            assert_eq!(true, o.is_match("b"));
            assert_eq!(false, o.is_match("c"));
            assert_eq!(false, o.is_match("aa"));
            assert_eq!(false, o.is_match(""));
        }
        {
            let a = Literal::new('a');
            let b = Literal::new('b');
            let c = Literal::new('c');
            let or1 = Or::new(&a, &b);
            let or2 = Or::new(&c, &or1);

            assert_eq!(true, or2.is_match("a"));
            assert_eq!(true, or2.is_match("b"));
            assert_eq!(true, or2.is_match("c"));
            assert_eq!(false, or2.is_match("d"));
            assert_eq!(false, or2.is_match("aa"));
        }
    }
}
