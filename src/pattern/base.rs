#![allow(dead_code)]

use crate::farule::FARule;
use std::vec::Vec;

pub trait BasePattern {
    fn is_match(&self, s: &str) -> bool;
    fn rule(&self) -> Vec<FARule>;
}
