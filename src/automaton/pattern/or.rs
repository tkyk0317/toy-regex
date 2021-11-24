#![allow(dead_code)]

use crate::automaton::farule::{FARule, State, TransitionType};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;
use std::boxed::Box;

#[derive(Debug)]
pub struct Or<T: BasePattern + ?Sized, U: BasePattern + ?Sized> {
    start_state: State,
    left: Box<T>,
    right: Box<U>,
}

impl<T: BasePattern + ?Sized, U: BasePattern + ?Sized> Or<T, U> {
    pub fn new(left: Box<T>, right: Box<U>) -> Self {
        Or {
            start_state: State::create_at_rnd(),
            left,
            right,
        }
    }
}

impl<T: BasePattern + ?Sized, U: BasePattern + ?Sized> BasePattern for Or<T, U> {
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
        let ep_to_l_rule = FARule::new(
            self.start_state,
            TransitionType::Epsilon,
            self.left.start_state(),
        );
        let ep_to_r_rule = FARule::new(
            self.start_state,
            TransitionType::Epsilon,
            self.right.start_state(),
        );

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
    use crate::automaton::pattern::literal::Literal;
    use std::boxed::Box;

    #[test]
    fn test_or() {
        {
            let l = Literal::new('a');
            let r = Literal::new('b');
            let o = Or::new(Box::new(l), Box::new(r));

            assert!(o.is_match("a"));
            assert!(o.is_match("b"));
            assert!(!o.is_match("c"));
            assert!(!o.is_match("aa"));
            assert!(!o.is_match(""));
        }
        {
            let a = Literal::new('a');
            let b = Literal::new('b');
            let c = Literal::new('c');
            let or1 = Or::new(Box::new(a), Box::new(b));
            let or2 = Or::new(Box::new(c), Box::new(or1));

            assert!(or2.is_match("a"));
            assert!(or2.is_match("b"));
            assert!(or2.is_match("c"));
            assert!(!or2.is_match("d"));
            assert!(!or2.is_match("aa"));
        }
    }
}
