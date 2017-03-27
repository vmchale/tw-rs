//! Module containing benchmarks
//! TODO and hopefully soon tests
#![allow(unused_imports)]

extern crate test;

#[cfg(test)]

// use super::*;
use test::test::Bencher;
use parse::parse_tweets;
use std::io::prelude::*;
use std::fs::File;

#[bench]
fn bench_parser(b: &mut Bencher) {
    let mut file = File::open("testdata/response")
        .expect("make sure you are in the right directory");
    let mut test_data_str = String::new();
    file.read_to_string(&mut test_data_str)
        .expect("File read failed");
    let test_data = test_data_str.as_bytes();
    b.iter(|| parse_tweets(test_data));
}
