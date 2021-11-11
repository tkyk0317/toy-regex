#![allow(dead_code)]

use crate::farule::{FARule, State};
use std::char;
use std::vec::Vec;

#[derive(Debug)]
pub struct DFARulebook {
    rules: Vec<FARule>,
}

impl DFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        DFARulebook { rules: rules }
    }

    pub fn next_state(&self, state: State, character: Option<char>) -> State {
        *self.rule_for(&state, character).follow()
    }

    fn rule_for(&self, state: &State, character: Option<char>) -> &FARule {
        self.rules
            .iter()
            .find(|r| r.applies_to(state, &character))
            .unwrap()
    }
}

struct DFA<'a> {
    current_state: State,
    accept_states: &'a Vec<State>,
    rulebook: &'a DFARulebook,
}

impl<'a> DFA<'a> {
    pub fn new(
        current_state: State,
        accept_states: &'a Vec<State>,
        rulebook: &'a DFARulebook,
    ) -> Self {
        DFA {
            current_state: current_state,
            accept_states: accept_states,
            rulebook: rulebook,
        }
    }

    pub fn accepting(&self) -> bool {
        self.accept_states
            .iter()
            .find(|s| **s == self.current_state)
            .is_some()
    }

    pub fn read_string(&mut self, s: &str) {
        s.chars().for_each(|c| {
            self.current_state = self.rulebook.next_state(self.current_state, Some(c))
        })
    }
}

pub struct DFADesign<'a> {
    start_state: State,
    accept_states: &'a Vec<State>,
    rulebook: &'a DFARulebook,
}

impl<'a> DFADesign<'a> {
    pub fn new(
        start_state: State,
        accept_states: &'a Vec<State>,
        rulebook: &'a DFARulebook,
    ) -> Self {
        DFADesign {
            start_state: start_state,
            accept_states: accept_states,
            rulebook: rulebook,
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut dfa = DFA::new(self.start_state, self.accept_states, self.rulebook);
        dfa.read_string(s);
        dfa.accepting()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::farule::TransitionType;

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

        assert_eq!(State::new(2), dfa_rule.next_state(State::new(1), Some('a')));
        assert_eq!(State::new(1), dfa_rule.next_state(State::new(1), Some('b')));
        assert_eq!(State::new(3), dfa_rule.next_state(State::new(2), Some('b')));
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

        assert_eq!(
            true,
            DFA::new(State::new(1), &vec![State::new(1), State::new(3)], &rule).accepting()
        );
        assert_eq!(
            true,
            DFA::new(State::new(1), &vec![State::new(1)], &rule).accepting()
        );
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
        let mut dfa = DFA::new(State::new(1), &accept_statuses, &rule);
        dfa.read_string("baaab");

        assert_eq!(true, dfa.accepting());
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
        assert_eq!(true, design.accept("baaab"));
    }
}
