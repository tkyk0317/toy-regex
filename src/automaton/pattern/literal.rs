#![allow(dead_code)]

use crate::automaton::farule::{FARule, State, TransitionType};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;
use std::char;

#[derive(Debug)]
pub struct Literal {
    character: char,
    start_state: State,
    accept_state: State,
}

impl Literal {
    pub fn new(c: char) -> Self {
        Literal {
            start_state: State::create_at_rnd(),
            accept_state: State::create_at_rnd(),
            character: c,
        }
    }
}

impl BasePattern for Literal {
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
        vec![FARule::new(
            self.start_state,
            TransitionType::Character(self.character),
            self.accept_state,
        )]
    }

    fn accept_state(&self) -> Vec<State> {
        vec![self.accept_state]
    }

    fn start_state(&self) -> State {
        self.start_state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal() {
        let l = Literal::new('a');

        assert_eq!(false, l.is_match(""));
        assert_eq!(true, l.is_match("a"));
        assert_eq!(false, l.is_match("b"));
        assert_eq!(false, l.is_match(" a"));
        assert_eq!(false, l.is_match("a "));
    }
}
