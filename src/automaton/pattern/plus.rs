#![allow(dead_code)]

use crate::automaton::farule::{FARule, State};
use crate::automaton::nfa::{NFADesign, NFARulebook};
use crate::automaton::pattern::base::BasePattern;
use crate::automaton::pattern::{concat::Concat, repeat::Repeat};
use std::boxed::Box;

#[derive(Debug)]
pub struct Plus<T: BasePattern + ?Sized> {
    pattern: Box<Concat<T, Repeat<T>>>,
}

impl<T: BasePattern + ?Sized> Plus<T> {
    pub fn new(l: Box<T>, r: Box<T>) -> Self {
        let repeat = Box::new(Repeat::new(r));
        Plus {
            pattern: Box::new(Concat::new(l, repeat)),
        }
    }
}

impl<T: BasePattern + ?Sized> BasePattern for Plus<T> {
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
    use crate::automaton::pattern::literal::Literal;

    #[test]
    fn test_plus() {
        let l = Box::new(Literal::new('a'));
        let r = Box::new(Literal::new('a'));
        let plus = Plus::new(l, r);

        assert_eq!(true, plus.is_match("a"));
        assert_eq!(true, plus.is_match("aa"));
        assert_eq!(true, plus.is_match("aaaaaaaaa"));
        assert_eq!(false, plus.is_match("ab"));
        assert_eq!(false, plus.is_match(""));
        assert_eq!(false, plus.is_match("ba"));
    }
}
