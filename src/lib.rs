#[macro_use] extern crate nom;

extern crate colored;

use nom::{IResult};
use std::fmt;
use colored::*;
use std::str::from_utf8;

pub struct Tweet<'a>{
    pub text: &'a[u8],
    pub name: &'a[u8],
    pub retweets: &'a[u8],
    pub favorites: &'a[u8],
}

impl<'a> fmt::Display for Tweet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let heart = "\u{1F49C}".red(); // \u{2665}
        let retweets = "\u{F079}".green(); // \u{267A}
        write!(f, "{}\n    {}\n    {} {} {}  {}\n", 
               from_utf8(self.name).unwrap().yellow(), 
               from_utf8(self.text).unwrap(),
               heart,
               from_utf8(self.favorites).unwrap(),
               retweets,
               from_utf8(self.retweets).unwrap())
    }
}

named!(prefield, delimited!(char!('"'), is_not!("\"\\"), char!('"'))); 
named!(field, alt!(prefield | unicode_char | special_char));
named!(int_field, take_until!(","));
named!(text_value,
  do_parse!(
    take_until!("\"text\"") >>
    tag!("\"text\":") >>
    value: field >>
    (value)
  )
);
named!(unicode_char,
  do_parse!(
    char!('\\') >>
    char!('u') >>
    num: take!(4) >>
    (num)
  )
);
//from_u32 is what we want I think
named!(special_char,
  do_parse!(
    char!('\\') >>
    value: take!(1) >>
    (value)
  )
);
named!(name_value,
  do_parse!(
    take_until!("\"name\"") >>
    tag!("\"name\":") >> // of course we want it to work more than once!
    value: field >>
    (value)
  )
);
//figure out how to efficiently do this: macros probably?
named!(retweets_value,
  do_parse!(
    take_until!("\"retweet_count\"") >>
    tag!("\"retweet_count\":") >>
    value: int_field >>
    (value)
  )
);
named!(favorites_value,
  do_parse!(
    take_until!("\"favorite_count\"") >>
    tag!("\"favorite_count\":") >>
    value: int_field >>
    (value)
  )
);
named!(step_parse<&[u8], Tweet >,
  do_parse!(
    get_text: text_value >>
    get_name: name_value >>
    get_retweets: retweets_value >>
    get_favorites: favorites_value >>
    (Tweet{text: get_text, name: get_name, retweets: get_retweets, favorites: get_favorites })
  )
);
named!(big_parser<&[u8], Vec<Tweet> > , many0!(step_parse)); 

// consider an example? long though that may be
/// Parse a string of bytes as a vector of tweets
pub fn parse_tweets(str_in: &[u8]) -> IResult<&[u8], Vec<Tweet>> {
    big_parser(str_in)
}

/// urls for the twitter api 
pub mod api {
    pub const USER_TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    pub const TIMELINE: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    pub const STATUS_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
}
