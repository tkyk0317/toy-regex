#![allow(dead_code)]

use crate::vm::build::{Builder, RegexIR};

// コンテキスト
struct Context {
    sp: usize, // string pointer
    pc: usize, // program counter
}

impl Context {
    pub fn clear(&mut self) {
        self.sp = 0;
        self.pc = 0;
    }
}

pub struct Machine {
    inst: Vec<RegexIR>,
    context: Context,
}

impl Machine {
    pub fn new(pattern: &str) -> Self {
        Machine {
            inst: Builder::new(pattern).compile(),
            context: Context { sp: 0, pc: 0 },
        }
    }

    // 仮想マシン実行
    pub fn start(&mut self, str: &str) -> bool {
        // context初期化
        self.context.clear();

        // 各命令を実行
        self.exec(&str.chars().collect::<Vec<char>>())
    }

    // 命令実行
    fn exec(&mut self, str: &[char]) -> bool {
        if self.context.pc >= self.inst.len() {
            return false;
        }
        let inst = &self.inst[self.context.pc];
        match inst {
            RegexIR::Char(regex_c) => {
                if self.context.sp >= str.len() {
                    return false;
                }
                if *regex_c != str[self.context.sp] {
                    return false;
                }
                self.context.pc += 1;
                self.context.sp += 1;

                self.exec(str)
            }
            RegexIR::Match => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_machine_only_char() {
        {
            let mut m = Machine::new("a");

            assert_eq!(true, m.start("a"));
            assert_eq!(true, m.start("aa"));
            assert_eq!(false, m.start("b"));
            assert_eq!(false, m.start(""));
        }
        {
            let mut m = Machine::new("abc");

            assert_eq!(true, m.start("abc"));
            assert_eq!(true, m.start("abcd"));
            assert_eq!(false, m.start("ab"));
            assert_eq!(false, m.start("a"));
            assert_eq!(false, m.start(""));
        }
    }
}
