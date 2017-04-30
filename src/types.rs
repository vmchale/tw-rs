//! This module contains the types for use by the tweet library. It implements Display, so you can
//! print it out easily. 

extern crate colored;

use std::str::from_utf8;
use std::fmt;
use self::colored::Colorize;
use std::env;

/// Struct encapsulating tweets. Includes the text, name of the user, number of retweets, and
/// number of favorites.
// TODO make this parse quoted too
#[derive(Clone)]
pub struct Tweet{
    pub text: String,
    pub name: String,
    pub retweets: String,
    pub favorites: String,
    pub id: String,
}

/// Same but don't bother converting to a string
pub struct TransientTweet<'a>{
    pub text: String,
    pub name: String,
    pub quoted: Option<TweetQuoted>,
    pub retweets: &'a[u8],
    pub favorites: &'a[u8],
    pub id: &'a[u8],
}

/// struct for tweet that was quoted
pub struct TweetQuoted{
    pub text: String,
    pub name: String,
}

pub fn convert(tweet: TransientTweet) -> Tweet {
    Tweet {text: tweet.text, name: tweet.name, retweets: from_utf8(tweet.retweets).expect("").to_string(), favorites: from_utf8(tweet.favorites).expect("").to_string(), id: from_utf8(tweet.id).expect("").to_string() }
}


/// Display formatter for our quoted tweets
impl fmt::Display for TweetQuoted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    {}\n    {}\n",
               self.name.yellow(),
               self.text)
    }
}

/// Display formatter for a tweet. To use without color, set the environment 
/// variable `CLICOLOR` to 0. To disable special symbol fonts, set the 
/// `DISABLE_EMOJI` environment variable.
impl<'a> fmt::Display for TransientTweet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (heart, retweets) = {
            if let Ok(_) = env::var("DISABLE_EMOJI")
                { 
                    ("\u{2665}".red(), "\u{267A}".green())
                }
            else {
                    ("\u{1F49C}".red(), "\u{F079}".green())
                }
        };
        if let Some(ref quote) = self.quoted {
            write!(f, "{}\n    {}\n    {} {} {}  {}\n{}", 
                   self.name.yellow(), 
                   self.text, 
                   heart,
                   from_utf8(self.favorites).unwrap(),
                   retweets,
                   from_utf8(self.retweets).unwrap(),
                   quote)
        }
        else {
            write!(f, "{}\n    {}\n    {} {} {}  {}\n", 
                   self.name.yellow(), 
                   self.text, 
                   heart,
                   from_utf8(self.favorites).unwrap(),
                   retweets,
                   from_utf8(self.retweets).unwrap())
        }
    }
}

impl<'a> fmt::Debug for TransientTweet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (heart, retweets) = {
            if let Ok(_) = env::var("DISABLE_EMOJI")
                { 
                    ("\u{2665}".red(), "\u{267A}".green())
                }
            else {
                    ("\u{1F49C}".red(), "\u{F079}".green())
                }
        };
        write!(f, "{} - {}\n    {}\n    {} {} {}  {}\n", 
               self.name.yellow(), 
               from_utf8(self.id).unwrap().underline().blue(),
               self.text, 
               heart,
               from_utf8(self.favorites).unwrap(),
               retweets,
               from_utf8(self.retweets).unwrap())
    }
}
