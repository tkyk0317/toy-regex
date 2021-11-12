#![allow(dead_code)]

use crate::farule::{FARule, State, TransitionType};
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

#[derive(Debug)]
pub struct Repeat<'a, T: BasePattern> {
    start_state: State,
    element: &'a T,
}

impl<'a, T: BasePattern> Repeat<'a, T> {
    pub fn new(element: &'a T) -> Self {
        Repeat {
            start_state: State::create_at_rnd(),
            element,
        }
    }
}

impl<'a, T: BasePattern> BasePattern for Repeat<'a, T> {
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
    use crate::pattern::literal::Literal;

    #[test]
    fn test_repeat() {
        let l = Literal::new('a');
        let r = Repeat::new(&l);

        assert_eq!(true, r.is_match(""));
        assert_eq!(true, r.is_match("a"));
        assert_eq!(true, r.is_match("aa"));
        assert_eq!(true, r.is_match("aaa"));
        assert_eq!(false, r.is_match("b"));
        assert_eq!(false, r.is_match("ba"));
        assert_eq!(false, r.is_match("aab"));
    }
}
