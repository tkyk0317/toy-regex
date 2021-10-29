#![allow(dead_code)]

use std::char;

#[derive(Debug)]
pub struct FARule {
    state: i32,
    character: Option<char>, // Optional型にして、ε遷移を表現
    next_state: i32,
}

impl FARule {
    pub fn new(state: i32, character: Option<char>, next_state: i32) -> Self {
        FARule {
            state: state,
            character: character,
            next_state: next_state,
        }
    }

    pub fn applies_to(&self, state: i32, character: &Option<char>) -> bool {
        self.state == state && self.character == *character
    }

    pub fn follow(&self) -> i32 {
        self.next_state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_applies_to() {
        {
            let rule = FARule::new(1, Some('a'), 2);
            assert_eq!(true, rule.applies_to(1, &Some('a')));
            assert_eq!(false, rule.applies_to(2, &Some('a')));
            assert_eq!(false, rule.applies_to(1, &None));
        }
        {
            let rule = FARule::new(1, None, 2);
            assert_eq!(false, rule.applies_to(1, &Some('a')));
            assert_eq!(false, rule.applies_to(2, &Some('a')));
            assert_eq!(true, rule.applies_to(1, &None));
        }
    }
}
