#![allow(dead_code)]

use crate::farule::FARule;
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;
use std::char;

struct Literal {
    rule: NFARulebook,
    accept_state: Vec<i32>,
}

impl Literal {
    pub fn new(c: char) -> Self {
        Literal {
            rule: NFARulebook::new(vec![FARule::new(1, Some(c), 2)]),
            accept_state: vec![2],
        }
    }
}

impl BasePattern for Literal {
    fn is_match(&self, s: &str) -> bool {
        NFADesign::new(1, &self.accept_state, &self.rule).accept(s)
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
