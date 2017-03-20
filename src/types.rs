extern crate colored;

use std::str::from_utf8;
use std::fmt;
use types::colored::Colorize;

pub struct Tweet<'a>{
    //pub text: &'a[u8],
    pub text: String,
    //pub name: &'a[u8],
    pub name: String,
    pub retweets: &'a[u8],
    pub favorites: &'a[u8],
}

// global variable controlling coloring?? 
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
