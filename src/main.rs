#[macro_use] extern crate url;
#[macro_use] extern crate nom;

extern crate oauth_client;

//use oauth_client::Token;
use nom::{IResult,anychar};
use std::str::from_utf8;
use std::vec::Vec;
use url::percent_encoding::{QUERY_ENCODE_SET, utf8_percent_encode};

// parsers etc. 
named!(field, delimited!(char!('"'), is_not!("\""), char!('"')));
named!(text_tag, delimited!(char!('"'), tag!("text"), char!('"'))); // should be able to parse \" though! 
named!(name_tag, delimited!(char!('"'), tag!("name"), char!('"')));
named!(skip_one<&[u8]>,
    do_parse!(
        anychar >>
        (Default::default())
    )
);
named!(text_value,
  do_parse!(
    text_tag >>
    char!(':') >>
    value:  field >>
    (value)
  )
);
named!(name_value,
  do_parse!(
    name_tag >>
    char!(':') >>
    value:  field >>
    (value)
  )
);
named!(step_parser, alt!(text_value | name_value | skip_one));
//many where return type is an option? 
/// Parser to return text of tweets and usernames associated to them. 
named!(big_parser<&[u8], Vec<&[u8]> > , many0!(step_parser));

fn main() {
    let api_key = oauth_client::Token::new("","");
    let token = oauth_client::Token::new("","");
    let percent_encoded = utf8_percent_encode("tweeting with @rustlang this is the future.", TWITTER_ENCODE_SET).collect::<String>();
    let percent_encoded_slice = percent_encoded.as_str();
    let bytes_raw = oauth_client::get(api::USER_TIMELINE, &api_key, Some(&token), None).unwrap();
    println!("url: {}", (("https://api.twitter.com/1.1/statuses/update.json?status=".to_string() + percent_encoded_slice).as_str()));
    //let post_response = oauth_client::post((api::STATUS_UPDATE.to_string() + percent_encoded_slice).as_str(), &api_key, Some(&token), None).unwrap();
    let resp = String::from_utf8(bytes_raw).unwrap();
    let bytes_slice = resp.as_bytes();
    let parsed_maybe = big_parser(bytes_slice);
    if let IResult::Done(_,parsed) = parsed_maybe {
        let mut vector_bytes = Vec::new();
        for i in 0..parsed.len() {
            vector_bytes.push(from_utf8(parsed[i]).unwrap());
        }
        vector_bytes.retain(|&s| s != "");
        let final_thing = str::replace(vector_bytes.join("\n").as_str(),"\\/","/");
        println!("returned: {}", final_thing);
    }
}

/// urls for the twitter api. 
mod api {
    pub const USER_TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    pub const TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    pub const STATUS_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json?status=";
}

define_encode_set! {
    /// The encode set for tweets. 
    pub TWITTER_ENCODE_SET = [QUERY_ENCODE_SET] | {'@'} //, '.'}
}
