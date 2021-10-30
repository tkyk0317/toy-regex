#![allow(dead_code)]

pub trait BasePattern {
    fn is_match(&self, s: &str) -> bool;
}
