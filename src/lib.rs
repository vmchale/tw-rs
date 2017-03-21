//! This crate provides an executable Command Line Iinterface Tweeter, as well as several functions to return
//! tweets and to tweet. 
//!
//! ```c
//! $ tw send "tweeting from the command line :)"
//! ```
//!
//! If you get stuck:
//!
//! ```c
//! tw --help
//! ```
#[macro_use] extern crate nom;

extern crate oauth_client_fix as oauth_client;
extern crate core;

use std::collections::HashMap;
use oauth_client::Token;
use nom::IResult;

pub mod parse;
pub mod types;

/// Reads credentials from a string, i.e. gets them from a file.
///
/// # Examples
///
/// Put the following into a file:
///
/// ```c
/// api-key: API_KEY_HERE
/// api-sec: API_SECRET_HERE
/// tok: OAUTH_TOKEN_HERE
/// tok-sec: TOKEN_SECRET_HERE
/// ```
pub fn get_credentials(contents: &str) -> (Token, Token) {
    let mut iter = contents.split_whitespace();
    iter.next();
    let api_key = iter.next().expect("");
    iter.next();
    let api_sec = iter.next().expect("");
    iter.next();
    let tok = iter.next().expect("");
    iter.next();
    let tok_sec = iter.next().expect("");
    let key = oauth_client::Token::new(api_key, api_sec);
    let token = oauth_client::Token::new(tok,tok_sec);
    (key, token)
}

/// Display the raw JSON of a response, useful for debugging.
pub fn profile_raw(api_key: Token, token: Token) {
    let mut param = HashMap::new();
    let _ = param.insert("screen_name".into(), "".into());
    let _ = param.insert("count".into(), "15".into()); // TODO accept number of tweets to get
    let bytes_raw = oauth_client::get(api::USER_PROFILE, &api_key, Some(&token), Some(&param)).unwrap();
    // convert vector of u8's to &[u8] (array slice)
    let resp = String::from_utf8(bytes_raw).unwrap();
    println!("response:\n{}", resp);
}

/// Display profile for a given user. Takes screen name and number of tweets to return as
/// parameters. 
///
/// Note that Twitter's API allow for a maximum of 3200 tweets at a time by this method. 
///
/// # Examples
/// 
/// ```
/// print_profile(realDonaldTrump, 100, API_KEY, TOKEN);
/// ```
pub fn print_profile(screen_name: &str, num: u8, api_key: Token, token: Token) {
    let mut param = HashMap::new();
    let num_str = num.to_string();
    let _ = param.insert("screen_name".into(), screen_name.into());
    let _ = param.insert("count".into(), num_str.into()); // TODO accept number of tweets to get
    let bytes_raw = oauth_client::get(api::USER_PROFILE, &api_key, Some(&token), Some(&param)).unwrap();
    // convert vector of u8's to &[u8] (array slice)
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        for i in 0..parsed.len() {
            println!("{}", parsed[i]);
        }
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// Send a tweet
///
/// # Examples
///
/// ```
/// tweet("having a good day :)", API_KEY, TOKEN);
/// ```
pub fn tweet(sent_text: &str, api_key: Token, token: Token) {
    let mut param = HashMap::new();
    let _ = param.insert("status".into(), sent_text.into());
    let bytes_raw = oauth_client::post(api::STATUS_UPDATE, &api_key, Some(&token), Some(&param)).unwrap();
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        for i in 0..parsed.len() {
            println!("{}", parsed[i]);
        }
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// Display timeline. Takes number of tweets to return as
/// a parameter. 
///
/// Note that Twitter's API allow for a maximum of 3200 tweets at a time by this method. 
///
/// # Examples
/// 
/// ```
/// print_timeline(5, API_KEY, TOKEN);
/// ```
pub fn print_timeline(num: u8, api_key: Token, token: Token) {
    let num_str = num.to_string();
    let mut param = HashMap::new();
    let _ = param.insert("count".into(), num_str.into()); 
    let bytes_raw = oauth_client::get(api::TIMELINE, &api_key, Some(&token), Some(&param)).unwrap();
    // convert vector of u8's to &[u8] (array slice)
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        for i in 0..parsed.len() {
            println!("{}", parsed[i]);
        }
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// urls for the twitter api 
pub mod api {
    pub const USER_PROFILE: &'static str = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    pub const TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    pub const STATUS_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
}
