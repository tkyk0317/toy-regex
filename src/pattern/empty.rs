#![allow(dead_code)]

use crate::farule::{FARule, State};
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

#[derive(Debug)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }
}

impl BasePattern for Empty {
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
            Some('\0'),
            State::create_at_rnd(),
        )]
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
