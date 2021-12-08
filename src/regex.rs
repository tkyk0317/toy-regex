#![allow(dead_code)]

use crate::automaton::pattern::build::Builder;
use crate::vm::machine::Machine;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Regex<'a> {
    pattern: &'a str,
}

impl<'a> Regex<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Regex { pattern }
    }

    // 正規表現実行
    pub fn exec(&self, input_str: Option<String>, vm: bool, substring: bool, input_file: Option<PathBuf>) -> bool {
        // 部分文字列マッチ対応
        let pattern = self.substring_pattern(substring);

        // 検索対象文字列読み込み
        match self.read_str(input_file, input_str) {
            Ok(str) => {
                if vm {
                    Machine::new(&pattern).is_match(&str)
                } else {
                    Builder::new(&pattern).to_dfa().is_match(&str)
                }
            },
            Err(e) => {
                println!("{:?}", e);
                false
            }
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

    // 対象文字列生成
    fn read_str(&self, input_file: Option<PathBuf>, input_str: Option<String>) -> Result<String, &str> {
        // ファイル読みこみ
        if let Some(f) = input_file {
            let mut fp = File::open(f).expect("[Regex::exec] not found file");
            let mut contents = String::new();
            fp.read_to_string(&mut contents).expect("[Regex::exec] read_to_string is error");

            Ok(contents)
        }
        else if let Some(s) = input_str {
            Ok(s)
        }
        else {
            Err("[Regex::read_str] no input string and file")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_at_dfa() {
        let re = Regex::new("a?bc");

        assert!(re.exec(Some("bc".to_string()), false, true, None));
        assert!(re.exec(Some("abc".to_string()), false, true, None));
        assert!(re.exec(Some("aabc".to_string()), false, true, None));
        assert!(!re.exec(Some("".to_string()), false, true, None));
        assert!(!re.exec(Some("ab".to_string()), false, true, None));
        assert!(!re.exec(Some("aab".to_string()), false, true, None));
    }

    #[test]
    fn test_exec_at_vm() {
        let re = Regex::new("abc");

        assert!(re.exec(Some("abc".to_string()), true, true, None));
        assert!(re.exec(Some("aabc".to_string()), true, true, None));
        assert!(!re.exec(Some("".to_string()), true, true, None));
        assert!(!re.exec(Some("ab".to_string()), true, true, None));
        assert!(!re.exec(Some("aab".to_string()), true, true, None));
    }

    #[test]
    fn test_exec_at_dfa_from_file() {
        let re = Regex::new("allow");

        let path = PathBuf::from("./src/main.rs");
        assert!(re.exec(None, false, true, Some(path)));
    }

    #[test]
    fn test_exec_at_vm_from_file() {
        let re = Regex::new("allow");

        let path = PathBuf::from("./src/main.rs");
        assert!(re.exec(None, true, true, Some(path)));
    }

    #[test]
    fn test_exec_no_input() {
        let re = Regex::new("allow");

        assert!(!re.exec(None, true, true, None));
    }
}
