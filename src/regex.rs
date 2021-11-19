#![allow(dead_code)]

use crate::automaton::pattern::build::Builder;
use crate::vm::machine::Machine;

#[derive(Debug)]
pub struct Regex<'a> {
    pattern: &'a str,
}

impl<'a> Regex<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Regex { pattern }
    }

    // 正規表現実行
    pub fn exec(&self, str: &str, vm: bool) -> bool {
        if vm {
            Machine::new(self.pattern).is_match(str)
        } else {
            Builder::new(self.pattern).to_dfa().is_match(str)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_at_dfa() {
        let re = Regex::new("a?bc");

        assert_eq!(true, re.exec("bc", false));
        assert_eq!(true, re.exec("abc", false));
        assert_eq!(false, re.exec("", false));
        assert_eq!(false, re.exec("ab", false));
        assert_eq!(false, re.exec("aab", false));
        assert_eq!(false, re.exec("aabc", false));
    }

    #[test]
    fn test_exec_at_vm() {
        let re = Regex::new("abc");

        assert_eq!(true, re.exec("abc", true));
        assert_eq!(false, re.exec("", true));
        assert_eq!(false, re.exec("ab", true));
        assert_eq!(false, re.exec("aab", true));
        assert_eq!(false, re.exec("aabc", true));
    }
}
