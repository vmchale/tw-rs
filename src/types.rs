//! This module contains the types for use by the tweet library. It implements Display, so you can
//! print it out easily. 

extern crate colored;

use std::str::from_utf8;
use std::fmt;
use self::colored::Colorize;

/// Struct encapsulating tweets. Includes the text, name of the user, number of retweets, and
/// number of favorites. 
#[derive(Clone)]
pub struct Tweet<'a>{
    pub text: String,
    pub name: String,
    pub retweets: &'a[u8],
    pub favorites: &'a[u8],
    pub id: &'a[u8],
}

// TODO global variable controlling coloring?? 
impl<'a> fmt::Display for Tweet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let heart = "\u{1F49C}".red(); // \u{2665}
        let retweets = "\u{F079}".green(); // \u{267A}
        write!(f, "{}\n    {}\n    {} {} {}  {}\n", 
               self.name.yellow(), 
               self.text, 
               heart,
               from_utf8(self.favorites).unwrap(),
               retweets,
               from_utf8(self.retweets).unwrap())
    }
}

impl<'a> fmt::Debug for Tweet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let heart = "\u{1F49C}".red(); // \u{2665}
        let retweets = "\u{F079}".green(); // \u{267A}
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
