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
//! tw help
//! ```
#[macro_use] extern crate nom;

extern crate oauth_client_fix as oauth_client;
extern crate core;
 extern crate base64;

use base64::encode;
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

/*
/// Return profile for a given user. 
pub fn get_profile<'a>(screen_name: &str, num: u8, api_key: Token, token: Token) -> Result<Vec<Tweet<'a>>,()> {
    let mut param = HashMap::new();
    let num_str = num.to_string();
    let _ = param.insert("screen_name".into(), screen_name.into());
    let _ = param.insert("count".into(), num_str.into()); // TODO accept number of tweets to get
    let bytes_raw = oauth_client::get(api::USER_PROFILE, &api_key, Some(&token), Some(&param)).unwrap();
    // convert vector of u8's to &[u8] (array slice)
    let bytestring = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = bytestring.as_bytes();
    // parse as an IResult
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    match parsed_maybe {
        IResult::Done(_,parsed) => Ok(parsed),
        _ => Err(panic!("Tweet failed to parse!")),
    }
}
*/

/// Display profile for a given user. Takes screen name and number of tweets to return as
/// parameters. Boolean argument is whether to print out user ids. 
///
/// Note that Twitter's API allow for a maximum of 3200 tweets at a time by this method. 
///
/// # Examples
/// 
/// ```
/// print_profile(realDonaldTrump, 100, false, API_KEY, TOKEN);
/// ```
pub fn print_profile(screen_name: &str, num: u8, show_ids: bool, api_key: Token, token: Token) {
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
            if show_ids {
                println!("{:?}", parsed[i]);
            }
            else
            {
                println!("{}", parsed[i]);
            }
        }
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

fn image_tweet(image: &[u8], sent_text: &str, api_key: Token, token: Token) {
    let mut param = HashMap::new();
    let _ = param.insert("media_data".into(), encode(image).into());
    let bytes_raw = oauth_client::post(api::STATUS_UPDATE, &api_key, Some(&token), Some(&param)).unwrap();
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::get_media_id(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        let media_id_str = String::from_utf8(parsed.to_vec()).unwrap();
        let mut paramt = HashMap::new();
        let _ = paramt.insert("status".into(), sent_text.into());
        let _ = paramt.insert("media_id".into(), sent_text.into());
        let bytes_rawt = oauth_client::post(api::STATUS_UPDATE, &api_key, Some(&token), Some(&paramt)).unwrap();
        let respt = String::from_utf8(bytes_rawt).unwrap();
        let bytes_slicet = respt.as_bytes();
        let parsed_maybet = parse::parse_tweets(bytes_slicet);
        if let IResult::Done(_,parsedt) = parsed_maybet {
            println!("{}", parsedt[0]);
        }
        else {
            println!("Parse error when attempting to read tweet data.");
        }
    }
    else {
        println!("Tweet upload failed")
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
        println!("{}", parsed[0]);
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// Reply to a tweet
///
/// # Examples
///
/// ```
/// reply("@friend that sounds like a good idea!", 844370958781579265, API_KEY, TOKEN);
/// ```
pub fn reply(sent_text: &str, reply_to: u64, api_key: Token, token: Token) {
    let mut param = HashMap::new();
    let reply_to_str = reply_to.to_string();
    let _ = param.insert("status".into(), sent_text.into());
    let _ = param.insert("in_reply_to_status_id".into(), reply_to_str.into());
    let bytes_raw = oauth_client::post(api::STATUS_UPDATE, &api_key, Some(&token), Some(&param)).unwrap();
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    // FIXME just pop it off so it's faster? we only want one.
    if let IResult::Done(_,parsed) = parsed_maybe {
        println!("{}", parsed[0]);
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// Display timeline. Takes number of tweets to return as
/// a parameter. Second argument is whether to display the id of the tweets.
///
/// Note that Twitter's API allow for a maximum of 3200 tweets at a time by this method. 
///
/// # Examples
/// 
/// ```
/// print_timeline(5, false, API_KEY, TOKEN);
/// ```
pub fn print_timeline(num: u8, show_ids:bool, api_key: Token, token: Token) {
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
            if show_ids {
                println!("{:?}", parsed[i]);
            }
            else
            {
                println!("{}", parsed[i]);
            }
        }
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// Delete a tweet by its id
pub fn delete_tweet(tweet_id: u64, api_key: Token, token: Token) {
	let tweet_id_str = tweet_id.to_string();
	let url = api::DELETE.to_string() + tweet_id_str.as_str() + ".json";
	let _ = oauth_client::post(url.as_str(), &api_key, Some(&token), None).unwrap();
	// we don't really care about the return value - TODO better message
	println!("Tweet deleted successfully!");
}
	
/// Rewteet a tweet by its id
pub fn retweet(tweet_id: u64, api_key: Token, token: Token) {
	let tweet_id_str = tweet_id.to_string();
	let url = api::RETWEET.to_string() + tweet_id_str.as_str() + ".json";
	let _ = oauth_client::post(url.as_str(), &api_key, Some(&token), None).unwrap();
	// we don't really care about the return value - TODO better message
	println!("Tweet retweeted successfully!");
}

/// Favorite a tweet by its id
pub fn favorite_tweet(tweet_id: u64, api_key: Token, token: Token) {
	let tweet_id_str = tweet_id.to_string();
    let mut param = HashMap::new();
    let _ = param.insert("id".into(), tweet_id_str.into());
    let bytes_raw = oauth_client::post(api::FAVORITE, &api_key, Some(&token), Some(&param)).unwrap();
    let resp = String::from_utf8(bytes_raw).unwrap(); // FIXME if this fails it's likely because it was already favorited
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        println!("{}\n    Favorited", parsed[0]);
    }
    else {
        println!("Parse error when attempting to read tweet data.");
    }
}

/// Unfavorite a tweet by its id
pub fn unfavorite_tweet(tweet_id: u64, api_key: Token, token: Token) {
	let tweet_id_str = tweet_id.to_string();
    let mut param = HashMap::new();
    let _ = param.insert("id".into(), tweet_id_str.into());
    let bytes_raw = oauth_client::post(api::UNFAVORITE, &api_key, Some(&token), Some(&param)).unwrap(); // FIXME if this fails it's likely because it wasn't already favorited
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = parse::parse_tweets(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        println!("{}\n    Unfavorited", parsed[0]);
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
    pub const RETWEET: &'static str = "https://api.twitter.com/1.1/statuses/retweet/";
    pub const DELETE: &'static str = "https://api.twitter.com/1.1/statuses/destroy/";
    pub const UPLOAD: &'static str = "https://upload.twitter.com/1.1/media/upload.json";
    pub const FAVORITE: &'static str = "https://api.twitter.com/1.1/favorites/create.json";
    pub const UNFAVORITE: &'static str = "https://api.twitter.com/1.1/favorites/destroy.json";
}
