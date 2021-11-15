#![allow(dead_code)]

use crate::automaton::farule::{FARule, State};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{empty::Empty, literal::Literal, or::Or};
use std::boxed::Box;

#[derive(Debug)]
pub struct Question {
    pattern: Or<Literal, Empty>,
}

impl Question {
    pub fn new(c: char) -> Self {
        Question {
            pattern: Or::new(Box::new(Literal::new(c)), Box::new(Empty::new())),
        }
    }
}

impl BasePattern for Question {
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
    use crate::automaton::pattern::concat::Concat;

    #[test]
    fn test_question() {
        // a?のテスト
        {
            let q = Question::new('a');

            assert_eq!(true, q.is_match("a"));
            assert_eq!(true, q.is_match(""));
            assert_eq!(false, q.is_match("aa"));
        }
        // a?bのテスト
        {
            let q = Question::new('a');
            let b = Literal::new('b');
            let c = Concat::new(Box::new(q), Box::new(b));

            assert_eq!(true, c.is_match("ab"));
            assert_eq!(true, c.is_match("b"));
            assert_eq!(false, c.is_match("aab"));
            assert_eq!(false, c.is_match(""));
        }
    }
}
