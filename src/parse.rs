//! This module contains the parser to turn a byte slice into a [TransientTweet](struct.TransientTweet.html)
use nom::IResult;
use nom::IResult::{Done};
use types::{TransientTweet, Tweet};
use std::str::from_utf8;
use core::char::from_u32;
use types::convert;

fn char_vector_to_string(v: Vec<char>) -> String {
    let s:String = v.into_iter().collect();
    s
}

// TODO consider making this a methd?
// HOT TAKE: oop is just functional programming where composition is backwards
fn replace_unicode(string: &str) -> char {
    let num_int = u32::from_str_radix(&string[0..4], 16)
        .expect("Failed to parses hexadecimal");
    if let Some(return_value) = from_u32(num_int) {
        return_value 
    }
    else {
        '�'
    }
}
named!(inner_char<&[u8], char>, alt!(unicode_char | newline_char | special_char | none_of!("\\\"")));
named!(prefield<&[u8], Vec<char> >, many0!(inner_char)); 
named!(field<&[u8], Vec<char> >, delimited!(char!('"'), prefield, char!('"')));
named!(int_field, take_until!(","));
named!(text_value<&[u8], Vec <char> >,
  do_parse!(
    take_until!("\"text\"") >>
    tag!("\"text\":") >>
    value: field >>
    (value)
  )
);
named!(tweet_id,
  do_parse!(
    take_until!("\"id\":") >>
    tag!("\"id\":") >>
    num_value: take_until!(",") >>
    (num_value)
  )
);
named!(skip_quote_status,
  do_parse!(
    take_until!("\"is_quote_status\"") >>
    tag!("\"is_quote_status\":true") >>
    value: take!(1) >> 
    retweets_value >>
    (value)
  )
);
named!(unicode_char<&[u8], char>,
  do_parse!(
    tag!("\\u") >>
    num: take!(4) >>
    //char.encode_utf8(&mut [0;4]).as_bytes()
    (replace_unicode(from_utf8(num).expect("Failed to convert to bytes. Bad!!")))
  )
);
named!(special_char<&[u8], char>,
  do_parse!(
    char!('\\') >>
    value: take!(1) >>
    (from_utf8(value).unwrap().chars().next().unwrap())
  )
);
named!(newline_char<&[u8], char>,
  do_parse!(
    tag!("\\n") >>
    ('\n')
  )
);
named!(name_value<&[u8], Vec<char> >,
  do_parse!(
    take_until!("\"name\"") >>
    tag!("\"name\":") >> // fix so it doesn't take the first 
    value: field >>
    (value)
  )
);
named!(retweets_value,
  do_parse!(
    take_until!("\"retweet_count\"") >>
    tag!("\"retweet_count\":") >>
    value: int_field >>
    (value)
  )
);
named!(media_id,
  do_parse!(
    take_until!("\"media_id\"") >>
    tag!("\"media_id\":") >>
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
// FIXME make it recursive? or not idk
// basically there's an "indices":[2,19] field that might be there
named!(skip_mentions<&[u8], () >,
  do_parse!(
    take_until!("\"user_mentions\"") >>
    tag!("\"user_mentions\":") >>
    alt!(tag!("[]")
      | delimited!(tag!("["), take_until!("}]") , tag!("}]"))) >> 
    ()
  )
);
//TODO also skip first rt if it's a quote status?  
named!(step_parse<&[u8], TransientTweet >,
  do_parse!(
    get_id: tweet_id >>
    get_text: text_value >>
    skip_mentions >>
    get_name: name_value >>
    opt!(skip_quote_status) >>
    get_retweets: retweets_value >>
    get_favorites: favorites_value >>
    (TransientTweet{text: char_vector_to_string(get_text), name: char_vector_to_string(get_name), retweets: get_retweets, favorites: get_favorites, id: get_id })
  )
);
named!(big_parser<&[u8], Vec<TransientTweet> > , many0!(step_parse)); 

/// Parse a slice of bytes as a vector of tweets. The input should be the JSON-formatted response
/// sent back by twitter. You can look at an example response
/// [here](https://dev.twitter.com/rest/reference/get/statuses/user_timeline).
///
/// The function returns an IResult, so you can pattern match to use it. 
pub fn parse_tweets(str_in: &[u8]) -> IResult<&[u8], Vec<TransientTweet>> {
    big_parser(str_in)
}

/// Return a Tweet, suitable for libraries etc.
pub fn parse_tweets_string(str_in: &[u8]) -> Option<Vec<Tweet>> {
    if let Done(_,val) = parse_tweets(str_in) {
        Some(val.into_iter().map(|x| convert(x)).collect())
    }
    else {
        None
    }

}

pub fn get_media_id(str_in: &[u8]) -> IResult<&[u8], &[u8]> {
    media_id(str_in)
}
