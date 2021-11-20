#![allow(dead_code)]

use crate::vm::build::{Builder, RegexIR};
use std::rc::Rc;

// コンテキスト
#[derive(Debug, Clone)]
struct Context {
    sp: usize,              // string pointer
    pc: usize,              // program counter
    inst: Rc<Vec<RegexIR>>, // instructions
    target: Rc<Vec<char>>,  // target string
}

impl Context {
    pub fn new(inst: Vec<RegexIR>, target: Vec<char>) -> Self {
        Context {
            pc: 0,
            sp: 0,
            inst: Rc::new(inst),
            target: Rc::new(target),
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
    pub fn is_match(&mut self, str: &str) -> bool {
        // 各命令を実行
        let ctx = Context::new(self.inst.clone(), str.chars().collect());
        Self::exec(ctx)
    }

    // 正規表現VM実行
    fn exec(mut ctx: Context) -> bool {
        loop {
            if ctx.pc >= ctx.inst.len() {
                return false;
            }

            match ctx.inst[ctx.pc] {
                RegexIR::AllChar if ctx.sp < ctx.target.len() => {
                    ctx.pc += 1;
                    ctx.sp += 1;
                }
                RegexIR::Char(c) if ctx.sp < ctx.target.len() && c == ctx.target[ctx.sp] => {
                    ctx.pc += 1;
                    ctx.sp += 1;
                }
                RegexIR::Jmp(x) => ctx.pc = x,
                RegexIR::Match => return true,
                // PC位置を変更し、スレッド起動
                RegexIR::Split(x, y) => return Self::thread(&ctx, x) | Self::thread(&ctx, y),
                _ => return false,
            };
        }
    }

    // スレッド起動
    fn thread(ctx: &Context, pc: usize) -> bool {
        let mut t_ctx = ctx.clone();
        t_ctx.pc = pc;
        Self::exec(t_ctx)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_machine_only_char() {
        {
            let mut m = Machine::new("a");

            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("aa"));
            assert_eq!(false, m.is_match("b"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("abc");

            assert_eq!(true, m.is_match("abc"));
            assert_eq!(true, m.is_match("abcd"));
            assert_eq!(false, m.is_match("ab"));
            assert_eq!(false, m.is_match("a"));
            assert_eq!(false, m.is_match(""));
        }
    }

    #[test]
    fn test_machine_plus() {
        {
            let mut m = Machine::new("a+");

            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("aa"));
            assert_eq!(false, m.is_match("b"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("a+b+");

            assert_eq!(true, m.is_match("ab"));
            assert_eq!(true, m.is_match("aabb"));
            assert_eq!(true, m.is_match("aaaaaab"));
            assert_eq!(true, m.is_match("aaaaaabbbbbbbbbbb"));
            assert_eq!(true, m.is_match("aabbc"));
            assert_eq!(false, m.is_match("a"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("a+b+c+d+e+");

            assert_eq!(true, m.is_match("abcde"));
            assert_eq!(true, m.is_match("aabbccddee"));
            assert_eq!(
                true,
                m.is_match("aaaaaaaaaaaabbbbbbbbbbbbcccccccccccccddddddddddddeeeeeeeeeeeeee")
            );
            assert_eq!(false, m.is_match("abcd"));
            assert_eq!(false, m.is_match(""));
        }
    }

    #[test]
    fn test_machine_repeat() {
        {
            let mut m = Machine::new("a*");

            assert_eq!(true, m.is_match(""));
            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("aa"));
            assert_eq!(true, m.is_match("aaaaaaaaaaaa"));
            assert_eq!(true, m.is_match("b"));
        }
        {
            let mut m = Machine::new("aa*");

            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("aa"));
            assert_eq!(true, m.is_match("aaaaaaaaaaaa"));
            assert_eq!(false, m.is_match("b"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("aa*bb*");

            assert_eq!(true, m.is_match("ab"));
            assert_eq!(true, m.is_match("aab"));
            assert_eq!(true, m.is_match("aabb"));
            assert_eq!(true, m.is_match("aaaaaaaaaaaabbbbbbbbbb"));
            assert_eq!(false, m.is_match("a"));
            assert_eq!(false, m.is_match("b"));
            assert_eq!(false, m.is_match("c"));
            assert_eq!(false, m.is_match(""));
        }
    }

    #[test]
    fn test_machine_or() {
        let mut m = Machine::new("a|b");

        assert_eq!(true, m.is_match("a"));
        assert_eq!(true, m.is_match("b"));
        assert_eq!(true, m.is_match("aa"));
        assert_eq!(true, m.is_match("bb"));
        assert_eq!(false, m.is_match("c"));
        assert_eq!(false, m.is_match(""));
    }

    #[test]
    fn test_machine_question() {
        {
            let mut m = Machine::new("a?");

            assert_eq!(true, m.is_match(""));
            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("aa"));
            assert_eq!(true, m.is_match("aaa"));
            assert_eq!(true, m.is_match("b"));
        }
        {
            let mut m = Machine::new("aa?");

            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("aa"));
            assert_eq!(true, m.is_match("aaa"));
            assert_eq!(false, m.is_match("b"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("aa?bb?");

            assert_eq!(true, m.is_match("ab"));
            assert_eq!(true, m.is_match("aabb"));
            assert_eq!(true, m.is_match("aabbb"));
            assert_eq!(false, m.is_match("aaabb"));
            assert_eq!(false, m.is_match(""));
        }
    }
    #[test]
    fn test_machine_dot() {
        {
            let mut m = Machine::new(".");

            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("b"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("..*");

            assert_eq!(true, m.is_match("a"));
            assert_eq!(true, m.is_match("b"));
            assert_eq!(true, m.is_match("aaaaa"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("a.");

            assert_eq!(true, m.is_match("aa"));
            assert_eq!(true, m.is_match("ab"));
            assert_eq!(false, m.is_match("a"));
            assert_eq!(false, m.is_match(""));
        }
        {
            let mut m = Machine::new("a.b.");

            assert_eq!(true, m.is_match("acbd"));
            assert_eq!(true, m.is_match("axbz"));
            assert_eq!(false, m.is_match("a"));
            assert_eq!(false, m.is_match("ab"));
            assert_eq!(false, m.is_match("acb"));
        }
        {
            let mut m = Machine::new("a.b.c?");

            assert_eq!(true, m.is_match("acbd"));
            assert_eq!(true, m.is_match("acbdc"));
            assert_eq!(true, m.is_match("acbdcc"));
            assert_eq!(false, m.is_match("acb"));
        }
        {
            let mut m = Machine::new("a.b.*c?");

            assert_eq!(true, m.is_match("azbd"));
            assert_eq!(true, m.is_match("acbdd"));
            assert_eq!(true, m.is_match("acbddddd"));
            assert_eq!(true, m.is_match("acbdddddc"));
            assert_eq!(true, m.is_match("azb"));
            assert_eq!(false, m.is_match("az"));
        }
    }
}
