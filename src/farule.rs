#![allow(dead_code)]

use rand::Rng;
use std::char;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct State {
    id: usize,
}

impl State {
    pub fn new(id: usize) -> Self {
        State { id: id }
    }

    // IDを指定せず、作成する
    pub fn create_at_rnd() -> Self {
        // TODO: 重複したIDを生成すると、状態遷移が正常に行われない可能性あり
        let mut rng = rand::thread_rng();

        State { id: rng.gen() }
    }
}

impl Copy for State {}

#[derive(Debug)]
pub struct FARule {
    pub state: State,
    pub character: Option<char>, // Optional型にして、ε遷移を表現
    pub next_state: State,
}

impl FARule {
    pub fn new(state: State, character: Option<char>, next_state: State) -> Self {
        FARule {
            state: state,
            character: character,
            next_state: next_state,
        }
    }

    pub fn applies_to(&self, state: &State, character: &Option<char>) -> bool {
        self.state.id == state.id && self.character == *character
    }

    pub fn follow(&self) -> &State {
        &self.next_state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_applies_to() {
        {
            let rule = FARule::new(State::new(1), Some('a'), State::new(2));
            assert_eq!(true, rule.applies_to(&State::new(1), &Some('a')));
            assert_eq!(false, rule.applies_to(&State::new(2), &Some('a')));
            assert_eq!(false, rule.applies_to(&State::new(1), &None));
        }
        {
            let rule = FARule::new(State::new(1), None, State::new(2));
            assert_eq!(false, rule.applies_to(&State::new(1), &Some('a')));
            assert_eq!(false, rule.applies_to(&State::new(2), &Some('a')));
            assert_eq!(true, rule.applies_to(&State::new(1), &None));
        }
    }
}
