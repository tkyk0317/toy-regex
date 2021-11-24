#![allow(dead_code)]

use crate::automaton::farule::{FARule, State};
use std::char;
use std::vec::Vec;

#[derive(Debug)]
pub struct DFARulebook {
    rules: Vec<FARule>,
}

impl DFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        DFARulebook { rules }
    }

    pub fn next_state(&self, state: State, character: Option<char>) -> Option<State> {
        self.rule_for(&state, character).map(|r| *r.follow())
    }

    fn rule_for(&self, state: &State, character: Option<char>) -> Option<&FARule> {
        self.rules.iter().find(|r| r.applies_to(state, &character))
    }
}

struct Dfa<'a> {
    current_state: State,
    accept_states: &'a [State],
    rulebook: &'a DFARulebook,
}

impl<'a> Dfa<'a> {
    pub fn new(
        current_state: State,
        accept_states: &'a [State],
        rulebook: &'a DFARulebook,
    ) -> Self {
        Dfa {
            current_state,
            accept_states,
            rulebook,
        }
    }

    pub fn accepting(&self) -> bool {
        self.accept_states.iter().any(|s| *s == self.current_state)
    }

    pub fn read_string(&mut self, s: &str) -> Result<(), &str> {
        for c in s.chars() {
            if let Some(next_state) = self.rulebook.next_state(self.current_state, Some(c)) {
                self.current_state = next_state
            } else {
                return Err("");
            }
        }

        Ok(())
    }
}

pub struct DFADesign<'a> {
    start_state: State,
    accept_states: &'a [State],
    rulebook: &'a DFARulebook,
}

impl<'a> DFADesign<'a> {
    pub fn new(start_state: State, accept_states: &'a [State], rulebook: &'a DFARulebook) -> Self {
        DFADesign {
            start_state,
            accept_states,
            rulebook,
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut dfa = Dfa::new(self.start_state, self.accept_states, self.rulebook);
        match dfa.read_string(s) {
            Ok(_) => dfa.accepting(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automaton::farule::TransitionType;

    #[test]
    fn test_next_state() {
        let dfa_rule = DFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(3)),
        ]);

        assert_eq!(
            Some(State::new(2)),
            dfa_rule.next_state(State::new(1), Some('a'))
        );
        assert_eq!(
            Some(State::new(1)),
            dfa_rule.next_state(State::new(1), Some('b'))
        );
        assert_eq!(
            Some(State::new(3)),
            dfa_rule.next_state(State::new(2), Some('b'))
        );
    }

    #[test]
    fn test_dfa_accepting() {
        let rule = DFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(3)),
        ]);

        assert!(Dfa::new(State::new(1), &vec![State::new(1), State::new(3)], &rule).accepting());
        assert!(Dfa::new(State::new(1), &vec![State::new(1)], &rule).accepting());
    }

    #[test]
    fn test_dfa_read_string() {
        let rule = DFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(3)),
        ]);
        let accept_statuses = vec![State::new(3)];
        let mut dfa = Dfa::new(State::new(1), &accept_statuses, &rule);
        let _ = dfa.read_string("baaab");

        assert!(dfa.accepting());
    }

    #[test]
    fn test_dfa_desgin() {
        let rule = DFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(3)),
        ]);
        let accept_statuses = vec![State::new(3)];
        let design = DFADesign::new(State::new(1), &accept_statuses, &rule);
        assert!(design.accept("baaab"));
    }
}
