#[macro_use] extern crate url;
#[macro_use] extern crate clap;

extern crate oauth_client;
extern crate libclit;
extern crate nom;

use std::collections::HashMap;
use std::vec::Vec;
use libclit::*;
use libclit::api as api;
use nom::{IResult};
use clap::App;

fn main() {
    //parse command-line options
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(command) = matches.subcommand_matches("user") {
        if let Some(user) = command.value_of("screen_name") {
            println!("{}",user);
        }
        println!("getting user timeline...");
    }
    //my_string.parse::<i32>().unwrap();
    let mut param = HashMap::new();
    let _ = param.insert("count".into(), "2".into());
    let api_key = oauth_client::Token::new("KYWlTUp3Gpa7AluQRGlagfJLC","5kPUdSEfGUHHUbzrvF8Bw2ok756ljexGGK40DcuyIpiFGSv5zI");
    let token = oauth_client::Token::new("739626641450635265-1o4PFsZgKiyzGCRvM5UCb25yMUjQzfU","jWORKOIiXHBAxJAzrhMUogzDi3BPz3eC7jY3g0i5ajjoN");
    let bytes_raw = oauth_client::get(api::USER_TIMELINE, &api_key, Some(&token), Some(&param)).unwrap();
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse_tweets(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        for i in 0..parsed.len() {
            println!("{}", parsed[i]);
        }
    }
}
