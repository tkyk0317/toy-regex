// cargo +nightly benchで実行
#![feature(test)]

extern crate test;
use toy_regex::regex;

static LEN: usize = 100000;

#[bench]
fn regex_vm_bench(b: &mut test::Bencher) {
    let s = String::from_utf8(vec![b'a'; LEN]).unwrap();
    b.iter(|| regex::Regex::new(".*a").exec(&s, true));
}

//--------------------------------------------------------------------------------------------
// let s = String::from_utf8(vec![b'a'; 1000]).unwrap();
// b.iter(|| regex::Regex::new(".*a").exec(&s, true));
//
// [std::thread]     test regex_vm_bench ... bench: 101,351,112 ns/iter (+/- 2,282,545)
// [loop version]    test regex_vm_bench ... bench:      21,543 ns/iter (+/- 385)
// [async_std::task] test regex_vm_bench ... bench:     426,289 ns/iter (+/- 3,633)
//
// let s = String::from_utf8(vec![b'a'; 500000000]).unwrap();
// b.iter(|| regex::Regex::new(".*a").exec(&s, true));
// [loop version]    test regex_vm_bench ... bench: 477,497,662 ns/iter (+/- 23,017,031)
//--------------------------------------------------------------------------------------------

#[bench]
fn regex_nfa_bench(b: &mut test::Bencher) {
    let s = String::from_utf8(vec![b'a'; LEN]).unwrap();
    b.iter(|| regex::Regex::new(".*a").exec(&s, false));
}
