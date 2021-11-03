#![allow(dead_code)]

use crate::farule::{FARule, State};
use std::char;
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug)]
pub struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        NFARulebook { rules: rules }
    }

    pub fn next_state(&self, states: &HashSet<State>, c: Option<char>) -> HashSet<State> {
        let mut next_states = vec![];
        states.iter().for_each(|s| {
            let mut t = self.rule_for(s, c);
            next_states.append(&mut t);
        });

        next_states.into_iter().collect::<HashSet<State>>()
    }

    fn rule_for(&self, s: &State, c: Option<char>) -> Vec<State> {
        self.rules
            .iter()
            .filter(|r| r.applies_to(s, &c))
            .map(|r| *r.follow())
            .collect::<Vec<State>>()
    }
}

struct NFA<'a> {
    current_state: HashSet<State>,
    accept_states: &'a Vec<State>,
    rulebook: &'a NFARulebook,
}

impl<'a> NFA<'a> {
    pub fn new(
        current_state: HashSet<State>,
        accept_states: &'a Vec<State>,
        rulebook: &'a NFARulebook,
    ) -> Self {
        NFA {
            current_state: current_state,
            accept_states: accept_states,
            rulebook: rulebook,
        }
    }

    pub fn accepting(&self) -> bool {
        self.current_state
            .iter()
            .find(|c| self.accept_states.iter().find(|a| a == c).is_some())
            .is_some()
    }

    pub fn read_string(&mut self, s: &str) {
        s.chars().for_each(|c| {
            // ε遷移を行ってから通常遷移
            self.trans_epsilon();
            self.current_state = self.rulebook.next_state(&self.current_state, Some(c));
        });

        // 読み込み完了後、ε遷移
        self.trans_epsilon();
    }

    fn trans_epsilon(&mut self) {
        // ε遷移の結果がサブセットにならなくなるまで遷移
        let epsilon = self.rulebook.next_state(&self.current_state, None);
        if epsilon.is_subset(&self.current_state) {
            return;
        }

        self.current_state.extend(epsilon);
        self.trans_epsilon();
    }
}

#[derive(Debug)]
pub struct NFADesign<'a> {
    start_state: State,
    accept_states: &'a Vec<State>,
    rulebook: &'a NFARulebook,
}

impl<'a> NFADesign<'a> {
    pub fn new(
        start_state: State,
        accept_states: &'a Vec<State>,
        rulebook: &'a NFARulebook,
    ) -> Self {
        NFADesign {
            start_state: start_state,
            accept_states: accept_states,
            rulebook: rulebook,
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut start_state = HashSet::new();
        start_state.insert(self.start_state);

        let mut nfa = NFA::new(start_state, self.accept_states, self.rulebook);
        nfa.read_string(s);
        nfa.accepting()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::farule::TransitionType;

    #[test]
    fn test_nfarulebook() {
        {
            let book = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
                FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
            ]);

            assert_eq!(
                vec![State::new(1), State::new(2)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), Some('b'))
            );
            assert_eq!(
                vec![State::new(1), State::new(2), State::new(3)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(
                    &vec![State::new(1), State::new(2)].into_iter().collect(),
                    Some('b')
                )
            );
            assert_eq!(
                vec![State::new(1), State::new(2), State::new(4)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(
                    &vec![State::new(1), State::new(3)].into_iter().collect(),
                    Some('b')
                )
            );
        }
        {
            let book = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            ]);

            assert_eq!(
                vec![State::new(2)].into_iter().collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), None)
            );
            assert_eq!(
                vec![State::new(2)].into_iter().collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), Some('a'))
            );
            assert_eq!(
                vec![].into_iter().collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), Some('b'))
            );
        }
        {
            let book = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(4)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(2)),
                FARule::new(State::new(4), TransitionType::Character('a'), State::new(5)),
                FARule::new(State::new(5), TransitionType::Character('a'), State::new(6)),
                FARule::new(State::new(6), TransitionType::Character('a'), State::new(4)),
            ]);

            assert_eq!(
                vec![State::new(2), State::new(4)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), None)
            );
        }
    }

    #[test]
    fn test_nfa_accepting() {
        let book = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
        ]);

        assert_eq!(
            false,
            NFA::new(
                vec![State::new(1)].into_iter().collect::<HashSet<State>>(),
                &vec![State::new(4)],
                &book
            )
            .accepting()
        );
        assert_eq!(
            true,
            NFA::new(
                vec![State::new(1), State::new(2), State::new(4)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                &vec![State::new(4)],
                &book
            )
            .accepting()
        );
    }

    #[test]
    fn test_nfa_read_string() {
        let book = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
        ]);

        {
            let accept_states = vec![State::new(4)];
            let mut nfa = NFA::new(
                vec![State::new(1)].into_iter().collect::<HashSet<State>>(),
                &accept_states,
                &book,
            );
            nfa.read_string("bab");

            assert_eq!(true, nfa.accepting());
        }
        {
            let accept_states = vec![State::new(4)];
            let mut nfa = NFA::new(
                vec![State::new(1)].into_iter().collect::<HashSet<State>>(),
                &accept_states,
                &book,
            );
            nfa.read_string("bbbbb");

            assert_eq!(true, nfa.accepting());
        }
    }

    #[test]
    fn test_nfa_design() {
        {
            let rule = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
                FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
            ]);

            let accept_statuses = vec![State::new(4)];
            let design = NFADesign::new(State::new(1), &accept_statuses, &rule);

            assert_eq!(true, design.accept("bab"));
            assert_eq!(true, design.accept("bbbbb"));
            assert_eq!(false, design.accept("bbabb"));
        }
        {
            let rule = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(4)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(2)),
                FARule::new(State::new(4), TransitionType::Character('a'), State::new(5)),
                FARule::new(State::new(5), TransitionType::Character('a'), State::new(6)),
                FARule::new(State::new(6), TransitionType::Character('a'), State::new(4)),
            ]);

            let accept_statuses = vec![State::new(2), State::new(4)];
            let design = NFADesign::new(State::new(1), &accept_statuses, &rule);

            assert_eq!(false, design.accept("a"));
            assert_eq!(true, design.accept("aa"));
            assert_eq!(true, design.accept("aaa"));
            assert_eq!(true, design.accept("aaaa"));
            assert_eq!(false, design.accept("aaaaa"));
            assert_eq!(true, design.accept("aaaaaa"));
            assert_eq!(true, design.accept("aaaaaa"));
        }
    }
}
