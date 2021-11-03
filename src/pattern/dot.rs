#![allow(dead_code)]

use crate::farule::{FARule, State, TransitionType};
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

#[derive(Debug)]
struct Dot {
    start_state: State,
    accept_state: State,
}

impl Dot {
    pub fn new() -> Self {
        Dot {
            start_state: State::create_at_rnd(),
            accept_state: State::create_at_rnd(),
        }
    }
}

impl BasePattern for Dot {
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
            TransitionType::Everything,
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
    fn test_dot() {
        let d = Dot::new();

        assert_eq!(true, d.is_match("a"));
        assert_eq!(true, d.is_match("d"));
        assert_eq!(false, d.is_match(""));
    }
}
