#![allow(dead_code)]

use crate::automaton::farule::{FARule, State, TransitionType};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;

#[derive(Debug)]
pub struct Dot {
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
    use crate::automaton::pattern::{concat::Concat, literal::Literal, repeat::Repeat};
    use std::boxed::Box;

    #[test]
    fn test_dot() {
        {
            let d = Dot::new();

            assert!(d.is_match("a"));
            assert!(d.is_match("d"));
            assert!(!d.is_match(""));
        }
        {
            let d = Dot::new();
            let r = Repeat::new(Box::new(d));

            assert!(r.is_match("a"));
            assert!(r.is_match("aaaaaaaaaaaaaa"));
            assert!(r.is_match("b"));
            assert!(r.is_match(""));
        }
        {
            let l = Literal::new('a');
            let d = Dot::new();
            let c = Concat::new(Box::new(l), Box::new(d));

            assert!(c.is_match("ab"));
            assert!(c.is_match("az"));
            assert!(!c.is_match("abc"));
            assert!(!c.is_match("a"));
            assert!(!c.is_match(""));
        }
    }
}
