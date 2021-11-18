#![allow(dead_code)]

use crate::automaton::pattern::build::Builder;
use crate::vm::machine::Machine;

#[derive(Debug)]
pub struct Regex {
    pattern: String,
}

impl Regex {
    pub fn new(pattern: String) -> Self {
        Regex { pattern }
    }

    // 正規表現実行
    pub fn exec(&self, str: String, vm: bool) -> bool {
        if vm {
            Machine::new(&self.pattern).is_match(&str)
        } else {
            Builder::new(&self.pattern).to_dfa().is_match(&str)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_at_dfa() {
        let re = Regex::new("a?bc".to_string());

        assert_eq!(true, re.exec("bc".to_string(), false));
        assert_eq!(true, re.exec("abc".to_string(), false));
        assert_eq!(false, re.exec("".to_string(), false));
        assert_eq!(false, re.exec("ab".to_string(), false));
        assert_eq!(false, re.exec("aab".to_string(), false));
        assert_eq!(false, re.exec("aabc".to_string(), false));
    }

    #[test]
    fn test_exec_at_vm() {
        let re = Regex::new("abc".to_string());

        assert_eq!(true, re.exec("abc".to_string(), true));
        assert_eq!(false, re.exec("".to_string(), true));
        assert_eq!(false, re.exec("ab".to_string(), true));
        assert_eq!(false, re.exec("aab".to_string(), true));
        assert_eq!(false, re.exec("aabc".to_string(), true));
    }
}
