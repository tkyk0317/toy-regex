// cargo +nightly benchで実行
#![feature(test)]

extern crate test;
use toy_regex::regex;

#[bench]
fn regex_vm_bench(b: &mut test::Bencher) {
    let s = String::from_utf8(vec![b'a'; 4000]).unwrap();
    b.iter(|| regex::Regex::new(".*a").exec(&s, true));
}

// let s = String::from_utf8(vec![b'a'; 1000]).unwrap();
// b.iter(|| regex::Regex::new(".*a").exec(&s, true));
//
// [std::thread]     test regex_vm_bench ... bench: 101,351,112 ns/iter (+/- 2,282,545)
// [async_std::task] test regex_vm_bench ... bench:     426,289 ns/iter (+/- 3,633)
