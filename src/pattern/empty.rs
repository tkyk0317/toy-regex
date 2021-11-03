#![allow(dead_code)]

use crate::farule::{FARule, State, TransitionType};
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

#[derive(Debug)]
pub struct Empty {
    start_state: State,
    accept_state: State,
}

impl Empty {
    pub fn new() -> Self {
        Empty {
            start_state: State::create_at_rnd(),
            accept_state: State::create_at_rnd(),
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
    fn test_empty() {
        let e = Empty::new();

        assert_eq!(true, e.is_match("\0"));
        assert_eq!(false, e.is_match("a"));
        assert_eq!(false, e.is_match(" a"));
        assert_eq!(false, e.is_match("a "));
    }
}
