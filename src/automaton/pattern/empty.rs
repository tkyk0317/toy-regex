#![allow(dead_code)]

use crate::automaton::farule::{FARule, State, TransitionType};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;

#[derive(Debug)]
pub struct Empty {
    start_state: State,
}

impl Empty {
    pub fn new() -> Self {
        Empty {
            start_state: State::create_at_rnd(),
        }
    }
}

impl BasePattern for Empty {
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
            TransitionType::Character('\0'),
            self.start_state,
        )]
    }

    fn accept_state(&self) -> Vec<State> {
        vec![self.start_state]
    }

    fn start_state(&self) -> State {
        self.start_state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let e = Empty::new();

        assert!(e.is_match("\0"));
        assert!(!e.is_match("a"));
        assert!(!e.is_match(" a"));
        assert!(!e.is_match("a "));
    }
}
