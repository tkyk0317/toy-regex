#![allow(dead_code)]

use crate::automaton::farule::{FARule, State};
use std::vec::Vec;

pub trait BasePattern {
    fn is_match(&self, s: &str) -> bool;
    fn rules(&self) -> Vec<FARule>;
    fn accept_state(&self) -> Vec<State>;
    fn start_state(&self) -> State;
}
