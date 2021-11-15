mod automaton;
mod parse;
mod regex;

use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "toy-regex", about = "Regular expression tool for learning")]
struct Opt {
    // 正規表現
    #[structopt(help = "regex pattern")]
    regex: String,

    // 入力文字列
    #[structopt(short = "s", help = "Input string")]
    input_str: String,

    // インプットファイル
    #[structopt(short = "i", help = "[future func] Input filename")]
    input_file: Option<PathBuf>,

    // 正規表現エンジン切り替え
    #[structopt(long, help = "[future func] Use vm engine")]
    vm: Option<bool>,
}

fn main() {
    // コマンドラインから正規表現エンジン作成
    let opt = Opt::from_args();
    let regex = Regex::new(opt.regex);

    // 正規表現実行
    println!("{:?}", regex.exec(opt.input_str));
}
