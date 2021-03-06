#![allow(dead_code)]

use crate::automaton::farule::{FARule, State, TransitionType};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;
use std::boxed::Box;

#[derive(Debug)]
pub struct Repeat<T: BasePattern + ?Sized> {
    start_state: State,
    element: Box<T>,
}

impl<T: BasePattern + ?Sized> Repeat<T> {
    pub fn new(element: Box<T>) -> Self {
        Repeat {
            start_state: State::create_at_rnd(),
            element,
        }
    }
}

impl<T: BasePattern + ?Sized> BasePattern for Repeat<T> {
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
        // 開始状態から要素開始状態へε遷移
        let ep_rule = FARule::new(
            self.start_state,
            TransitionType::Epsilon,
            self.element.start_state(),
        );

        // 各ノードのルールを結合
        let mut rules = vec![ep_rule];
        rules.extend(self.element.rules());

        // 要素の受理状態から開始状態へのε遷移を追加
        self.element.accept_state().into_iter().for_each(|s| {
            let ep_rule = FARule::new(s, TransitionType::Epsilon, self.element.start_state());
            rules.push(ep_rule);
        });
        rules
    }

    fn accept_state(&self) -> Vec<State> {
        // 開始状態と要素の受理状態を登録
        let mut accepts = vec![self.start_state];
        accepts.extend(self.element.accept_state());
        accepts
    }

    fn start_state(&self) -> State {
        self.start_state
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automaton::pattern::literal::Literal;

    #[test]
    fn test_repeat() {
        let l = Literal::new('a');
        let r = Repeat::new(Box::new(l));

        assert!(r.is_match(""));
        assert!(r.is_match("a"));
        assert!(r.is_match("aa"));
        assert!(r.is_match("aaa"));
        assert!(!r.is_match("b"));
        assert!(!r.is_match("ba"));
        assert!(!r.is_match("aab"));
    }
}
