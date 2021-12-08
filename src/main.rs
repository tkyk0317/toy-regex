#![allow(dead_code)]

use std::path::PathBuf;
use structopt::StructOpt;
use toy_regex::regex::Regex;

#[derive(Debug, StructOpt)]
#[structopt(name = "toy-regex", about = "Regular expression tool for learning")]
struct Opt {
    // 正規表現
    #[structopt(help = "regex pattern")]
    regex: String,

    // 入力文字列
    #[structopt(short = "s", help = "Input string")]
    input_str: Option<String>,

    // インプットファイル
    #[structopt(short = "i", help = "[future func] Input filename")]
    input_file: Option<PathBuf>,

    // 正規表現エンジン切り替え
    #[structopt(long, help = "Use vm engine")]
    vm: bool,

    // 部分文字でマッチ
    #[structopt(
        long,
        parse(try_from_str),
        help = "match substring",
        default_value = "true"
    )]
    substring: bool,
}

fn main() {
    // コマンドラインから正規表現エンジン作成
    let opt = Opt::from_args();

    // 正規表現実行
    println!(
        "{:?}",
        Regex::new(&opt.regex).exec(opt.input_str, opt.vm, opt.substring, opt.input_file)
    );
}
