#![allow(dead_code)]

use crate::automaton::farule::{FARule, State};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{concat::Concat, literal::Literal, repeat::Repeat};
use std::boxed::Box;

#[derive(Debug)]
pub struct Plus {
    pattern: Concat<Literal, Repeat<Literal>>,
}

impl Plus {
    pub fn new(c: char) -> Self {
        let l1 = Box::new(Literal::new(c));
        let l2 = Box::new(Literal::new(c));
        Plus {
            pattern: Concat::new(l1, Box::new(Repeat::new(l2))),
        }
    }
}

impl BasePattern for Plus {
    fn is_match(&self, s: &str) -> bool {
        let rules = self.rules();
        NFADesign::new(
            self.start_state(),
            &self.accept_state(),
            &NFARulebook::new(rules),
        )
        .accept(s)
    }

    fn rules(&self) -> Vec<FARule> {
        self.pattern.rules()
    }

    fn accept_state(&self) -> Vec<State> {
        self.pattern.accept_state()
    }

    fn start_state(&self) -> State {
        self.pattern.start_state()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus() {
        let p = Plus::new('a');

        assert_eq!(true, p.is_match("a"));
        assert_eq!(true, p.is_match("aa"));
        assert_eq!(true, p.is_match("aaaaaaaaa"));
        assert_eq!(false, p.is_match("ab"));
        assert_eq!(false, p.is_match(""));
        assert_eq!(false, p.is_match("ba"));
    }
}
