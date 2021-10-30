#![allow(dead_code)]

use crate::farule::FARule;
use crate::nfa::{NFADesign, NFARulebook};
use crate::pattern::base::BasePattern;

struct Empty {
    rule: NFARulebook,
    accept_state: Vec<i32>,
}

impl Empty {
    pub fn new() -> Self {
        Empty {
            rule: NFARulebook::new(vec![FARule::new(1, Some('\0'), 2)]),
            accept_state: vec![2],
        }
    }
}

impl BasePattern for Empty {
    fn is_match(&self, s: &str) -> bool {
        NFADesign::new(1, &self.accept_state, &self.rule).accept(s)
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
