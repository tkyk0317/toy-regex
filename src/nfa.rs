#![allow(dead_code)]

use crate::farule::FARule;
use std::char;
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug)]
struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        NFARulebook { rules: rules }
    }

    pub fn next_state(&self, states: &HashSet<i32>, c: Option<char>) -> HashSet<i32> {
        let mut next_states = vec![];
        states.iter().for_each(|s| {
            let mut t = self.rule_for(*s, c);
            next_states.append(&mut t);
        });

        next_states.into_iter().collect::<HashSet<i32>>()
    }

    fn rule_for(&self, s: i32, c: Option<char>) -> Vec<i32> {
        self.rules
            .iter()
            .filter(|r| r.applies_to(s, &c))
            .map(|r| r.follow())
            .collect::<Vec<i32>>()
    }
}

struct NFA<'a> {
    current_state: HashSet<i32>,
    accept_states: &'a Vec<i32>,
    rulebook: &'a NFARulebook,
}

impl<'a> NFA<'a> {
    pub fn new(
        current_state: HashSet<i32>,
        accept_states: &'a Vec<i32>,
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
            let epsilon = self.rulebook.next_state(&self.current_state, None);
            self.current_state.extend(&epsilon);
            self.current_state = self.rulebook.next_state(&self.current_state, Some(c));
        })
    }
}

#[derive(Debug)]
struct NFADesign<'a> {
    start_state: i32,
    accept_states: &'a Vec<i32>,
    rulebook: &'a NFARulebook,
}

impl<'a> NFADesign<'a> {
    fn new(start_state: i32, accept_states: &'a Vec<i32>, rulebook: &'a NFARulebook) -> Self {
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

    #[test]
    fn test_nfarulebook() {
        {
            let book = NFARulebook::new(vec![
                FARule::new(1, Some('a'), 1),
                FARule::new(1, Some('b'), 1),
                FARule::new(1, Some('b'), 2),
                FARule::new(2, Some('a'), 3),
                FARule::new(2, Some('b'), 3),
                FARule::new(3, Some('a'), 4),
                FARule::new(3, Some('b'), 4),
            ]);

            assert_eq!(
                vec![1, 2].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1].into_iter().collect(), Some('b'))
            );
            assert_eq!(
                vec![1, 3].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1, 2].into_iter().collect(), Some('a'))
            );
            assert_eq!(
                vec![1, 2, 4].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1, 3].into_iter().collect(), Some('b'))
            );
        }
        {
            let book =
                NFARulebook::new(vec![FARule::new(1, None, 2), FARule::new(1, Some('a'), 2)]);

            assert_eq!(
                vec![2].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1].into_iter().collect(), None)
            );
            assert_eq!(
                vec![2].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1].into_iter().collect(), Some('a'))
            );
            assert_eq!(
                vec![].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1].into_iter().collect(), Some('b'))
            );
        }
        {
            let book = NFARulebook::new(vec![
                FARule::new(1, None, 2),
                FARule::new(1, None, 4),
                FARule::new(2, Some('a'), 3),
                FARule::new(3, Some('a'), 2),
                FARule::new(4, Some('a'), 5),
                FARule::new(5, Some('a'), 6),
                FARule::new(6, Some('a'), 4),
            ]);

            assert_eq!(
                vec![2, 4].into_iter().collect::<HashSet<i32>>(),
                book.next_state(&vec![1].into_iter().collect(), None)
            );
        }
    }

    #[test]
    fn test_nfa_accepting() {
        let book = NFARulebook::new(vec![
            FARule::new(1, Some('a'), 1),
            FARule::new(1, Some('b'), 1),
            FARule::new(1, Some('b'), 2),
            FARule::new(2, Some('a'), 3),
            FARule::new(2, Some('b'), 3),
            FARule::new(3, Some('a'), 4),
            FARule::new(3, Some('b'), 4),
        ]);

        assert_eq!(
            false,
            NFA::new(
                vec![1].into_iter().collect::<HashSet<i32>>(),
                &vec![4],
                &book
            )
            .accepting()
        );
        assert_eq!(
            true,
            NFA::new(
                vec![1, 2, 4].into_iter().collect::<HashSet<i32>>(),
                &vec![4],
                &book
            )
            .accepting()
        );
    }

    #[test]
    fn test_nfa_read_string() {
        let book = NFARulebook::new(vec![
            FARule::new(1, Some('a'), 1),
            FARule::new(1, Some('b'), 1),
            FARule::new(1, Some('b'), 2),
            FARule::new(2, Some('a'), 3),
            FARule::new(2, Some('b'), 3),
            FARule::new(3, Some('a'), 4),
            FARule::new(3, Some('b'), 4),
        ]);

        {
            let accept_states = vec![4];
            let mut nfa = NFA::new(
                vec![1].into_iter().collect::<HashSet<i32>>(),
                &accept_states,
                &book,
            );
            nfa.read_string("bab");

            assert_eq!(true, nfa.accepting());
        }
        {
            let accept_states = vec![4];
            let mut nfa = NFA::new(
                vec![1].into_iter().collect::<HashSet<i32>>(),
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
                FARule::new(1, Some('a'), 1),
                FARule::new(1, Some('b'), 1),
                FARule::new(1, Some('b'), 2),
                FARule::new(2, Some('a'), 3),
                FARule::new(2, Some('b'), 3),
                FARule::new(3, Some('a'), 4),
                FARule::new(3, Some('b'), 4),
            ]);

            let accept_statuses = vec![4];
            let design = NFADesign::new(1, &accept_statuses, &rule);

            assert_eq!(true, design.accept("bab"));
            assert_eq!(true, design.accept("bbbbb"));
            assert_eq!(false, design.accept("bbabb"));
        }
        {
            let rule = NFARulebook::new(vec![
                FARule::new(1, None, 2),
                FARule::new(1, None, 4),
                FARule::new(2, Some('a'), 3),
                FARule::new(3, Some('a'), 2),
                FARule::new(4, Some('a'), 5),
                FARule::new(5, Some('a'), 6),
                FARule::new(6, Some('a'), 4),
            ]);

            let accept_statuses = vec![2, 4];
            let design = NFADesign::new(1, &accept_statuses, &rule);

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
