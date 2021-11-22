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
    pub fn exec(&self, str: &str, vm: bool, substring: bool) -> bool {
        // 部分文字列マッチ対応
        let pattern = self.substring_pattern(substring);

        if vm {
            Machine::new(&pattern).is_match(str)
        } else {
            Builder::new(&pattern).to_dfa().is_match(str)
        }
    }

    // 部分文字列へマッチする正規表現を作成
    fn substring_pattern(&self, substring: bool) -> String {
        if substring {
            format!(".*{}.*", &self.pattern)
        } else {
            self.pattern.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_at_dfa() {
        let re = Regex::new("a?bc");

        assert_eq!(true, re.exec("bc", false, true));
        assert_eq!(true, re.exec("abc", false, true));
        assert_eq!(true, re.exec("aabc", false, true));
        assert_eq!(false, re.exec("", false, true));
        assert_eq!(false, re.exec("ab", false, true));
        assert_eq!(false, re.exec("aab", false, true));
    }

    #[test]
    fn test_exec_at_vm() {
        let re = Regex::new("abc");

        assert_eq!(true, re.exec("abc", true, true));
        assert_eq!(true, re.exec("aabc", true, true));
        assert_eq!(false, re.exec("", true, true));
        assert_eq!(false, re.exec("ab", true, true));
        assert_eq!(false, re.exec("aab", true, true));
    }
}
