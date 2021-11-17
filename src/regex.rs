#![allow(dead_code)]

use crate::automaton::pattern::build::Builder;

#[derive(Debug)]
pub struct Regex {
    pattern: String,
}

impl Regex {
    pub fn new(pattern: String) -> Self {
        Regex { pattern }
    }

    // 正規表現実行
    pub fn exec(&self, str: String) -> bool {
        Builder::new(&self.pattern).to_dfa().is_match(&str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec() {
        let re = Regex::new("a?bc".to_string());

        assert_eq!(true, re.exec("bc".to_string()));
        assert_eq!(true, re.exec("abc".to_string()));
        assert_eq!(false, re.exec("".to_string()));
        assert_eq!(false, re.exec("ab".to_string()));
        assert_eq!(false, re.exec("aab".to_string()));
        assert_eq!(false, re.exec("aabc".to_string()));
    }
}
