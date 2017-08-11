//! Module containing benchmarks
#![allow(unused_imports)]

#![feature(test)]
extern crate tweet;

#[cfg(test)]
extern crate test;

use test::test::Bencher;
use tweet::parse::parse_tweets;
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
