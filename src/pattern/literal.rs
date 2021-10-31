#![allow(dead_code)]

use crate::farule::{FARule, State};
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;
use std::char;

#[derive(Debug)]
pub struct Literal {
    character: Option<char>,
}

impl Literal {
    pub fn new(c: char) -> Self {
        Literal { character: Some(c) }
    }
}

impl BasePattern for Literal {
    fn is_match(&self, s: &str) -> bool {
        let rule = self.rule();
        NFADesign::new(
            rule[0].state,
            &vec![rule[0].next_state],
            &NFARulebook::new(rule),
        )
        .accept(s)
    }

    fn rule(&self) -> Vec<FARule> {
        vec![FARule::new(
            State::create_at_rnd(),
            self.character,
            State::create_at_rnd(),
        )]
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
