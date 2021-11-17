#![allow(dead_code)]

use crate::vm::build::{Builder, RegexIR};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

// コンテキスト
#[derive(Debug, Clone)]
struct Context {
    sp: usize,               // string pointer
    pc: usize,               // program counter
    inst: Arc<Vec<RegexIR>>, // instructions
    target: Arc<Vec<char>>,  // target string
}

impl Context {
    pub fn new(inst: Vec<RegexIR>, target: Vec<char>) -> Self {
        Context {
            pc: 0,
            sp: 0,
            inst: Arc::new(inst),
            target: Arc::new(target),
        }
    }

    pub fn clear(&mut self) {
        self.sp = 0;
        self.pc = 0;
    }
}

pub struct Machine {
    inst: Vec<RegexIR>,
}

impl Machine {
    pub fn new(pattern: &str) -> Self {
        Machine {
            inst: Builder::new(pattern).compile(),
        }
    }

    // 仮想マシン実行
    pub fn start(&mut self, str: &str) -> bool {
        // 各命令を実行
        let context = Context::new(self.inst.clone(), str.chars().collect());
        Self::exec(context)
    }

    // 命令実行
    fn exec(mut context: Context) -> bool {
        if context.pc >= context.inst.len() {
            return false;
        }

        // 各命令を実行
        match context.inst[context.pc] {
            RegexIR::Char(regex_c) => {
                if context.sp >= context.target.len() || regex_c != context.target[context.sp] {
                    return false;
                }
                context.pc += 1;
                context.sp += 1;

                Self::exec(context)
            }
            RegexIR::Split(x, y) => {
                // PC位置を変更し、スレッド起動
                let t1 = Self::thread(&context, x);
                let t2 = Self::thread(&context, y);

                t1.join().unwrap() | t2.join().unwrap()
            }
            RegexIR::Match => true,
        }
    }

    // スレッド起動
    fn thread(context: &Context, pc: usize) -> JoinHandle<bool> {
        let mut t_context = context.clone();
        t_context.pc = pc;
        spawn(move || Self::exec(t_context))
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

    #[test]
    fn test_machine_plus() {
        {
            let mut m = Machine::new("a+");

            assert_eq!(true, m.start("a"));
            assert_eq!(true, m.start("aa"));
            assert_eq!(false, m.start("b"));
            assert_eq!(false, m.start(""));
        }
        {
            let mut m = Machine::new("a+b+");

            assert_eq!(true, m.start("ab"));
            assert_eq!(true, m.start("aabb"));
            assert_eq!(true, m.start("aaaaaab"));
            assert_eq!(true, m.start("aaaaaabbbbbbbbbbb"));
            assert_eq!(true, m.start("aabbc"));
            assert_eq!(false, m.start("a"));
            assert_eq!(false, m.start(""));
        }
        {
            let mut m = Machine::new("a+b+c+d+e+");

            assert_eq!(true, m.start("abcde"));
            assert_eq!(true, m.start("aabbccddee"));
            assert_eq!(
                true,
                m.start("aaaaaaaaaaaabbbbbbbbbbbbcccccccccccccddddddddddddeeeeeeeeeeeeee")
            );
            assert_eq!(false, m.start("abcd"));
            assert_eq!(false, m.start(""));
        }
    }
}
