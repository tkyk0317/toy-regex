#![allow(dead_code)]

use rand::Rng;
use std::char;

// ステータス
#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

// 遷移タイプ
#[derive(Debug, PartialEq)]
pub enum TransitionType {
    Character(char), // 通常の文字
    Epsilon,         // イプシロン遷移
    Everything,      // 全ての文字を遷移
}

// 有限オートマトンルール
#[derive(Debug, PartialEq)]
pub struct FARule {
    pub state: State,
    pub transition: TransitionType,
    pub next_state: State,
}

impl FARule {
    pub fn new(state: State, transition: TransitionType, next_state: State) -> Self {
        FARule {
            state: state,
            transition: transition,
            next_state: next_state,
        }
    }

    pub fn applies_to(&self, state: &State, character: &Option<char>) -> bool {
        match *character {
            Some(c1) => match self.transition {
                TransitionType::Character(c2) => self.state.id == state.id && c1 == c2,
                TransitionType::Everything => self.state.id == state.id,
                _ => false,
            },
            None => match self.transition {
                TransitionType::Epsilon => self.state.id == state.id,
                _ => false,
            },
        }
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
            let rule = FARule::new(State::new(1), TransitionType::Character('a'), State::new(2));
            assert_eq!(true, rule.applies_to(&State::new(1), &Some('a')));
            assert_eq!(false, rule.applies_to(&State::new(2), &Some('a')));
            assert_eq!(false, rule.applies_to(&State::new(1), &None));
        }
        {
            let rule = FARule::new(State::new(1), TransitionType::Epsilon, State::new(2));
            assert_eq!(false, rule.applies_to(&State::new(1), &Some('a')));
            assert_eq!(false, rule.applies_to(&State::new(2), &Some('a')));
            assert_eq!(true, rule.applies_to(&State::new(1), &None));
        }
    }
}
