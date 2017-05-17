#[macro_use] extern crate clap;

extern crate oauth_client_fix as oauth_client;
extern crate tweet;
extern crate nom;
extern crate colored;

use tweet::*;
use clap::App;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::io;
use colored::*;

fn main() {
    
    // command-line parser
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // set path to credentials file
    let path_read = matches.value_of("credentials");
    let path = 
        if let Some(read) = path_read {
            let mut path_in = PathBuf::new();
            path_in.push(read);
            path_in
        }
        else {
            // default path is ~/.cred
            let mut path_in = std::env::home_dir().expect("Couldn't determine home directory! Please enter path to credentials with -c or --cred.");
            path_in.push(".cred");
            path_in
        };

    // decide whether to print ids
    let show_ids  = matches.occurrences_of("show") == 1 ;

    // read api keys
    let mut file = File::open(path)
        .expect("File could not be read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("File read failed");
    let (key, token) = get_credentials(&contents);

    // print a cute bird
    let birb = "\u{1F426}".blue();
    println!("{}\n", birb);

    // get user timeline
    if let Some(command) = matches.subcommand_matches("user") {
        if let Some(user) = command.value_of("screen_name") {
            if let Some(num) = command.value_of("count") {
                let num_int = num.parse::<u8>()
                    .expect("Please enter a positive whole number");
                print_profile(user, num_int, show_ids, &key, &token);
            }
            else {
                // default is to fetch 8 tweets from the profile
                print_profile(user, 8, show_ids, &key, &token);
            }
        }
        else if let Some(num) = command.value_of("count") {
                let num_int = num.parse::<u8>()
                    .expect("Please enter a positive whole number");
                print_profile("", num_int, show_ids, &key, &token);
            }
        else {
            // this will return the user's own profile
            print_profile("", 8, show_ids, &key, &token); 
        }
    }

    // view timeline
    else if let Some(command) = matches.subcommand_matches("view") {
        if let Some(num) = command.value_of("count") {
            let num_int = num.parse::<u8>()
                .expect("Please enter a positive whole number");
            print_timeline(num_int, show_ids, &key, &token);
        }
        else {
            // default is to fetch 15 tweets from the user's timeline
            print_timeline(15, show_ids, &key, &token);
        }
    }

    // send a tweet
    else if let Some(command) = matches.subcommand_matches("send") {
        let send_str = command.value_of("words")
            .expect("Could not parse user input. Please check the string is correctly formatted");
        tweet(send_str, &key, &token);
    }

    // follow a user
    else if let Some(command) = matches.subcommand_matches("follow") {
        let screen_name = command.value_of("screen_name")
            .expect("Could not parse user input. Please check the string is correctly formatted");
        follow(screen_name, &key, &token);
    }

    // unfollow a user
    else if let Some(command) = matches.subcommand_matches("unfollow") {
        let screen_name = command.value_of("screen_name")
            .expect("Could not parse user input. Please check the string is correctly formatted");
        unfollow(screen_name, &key, &token);
    }

    // send a tweet from stdin (i.e. a pipe)
    else if matches.subcommand_matches("input").is_some() {
        let mut buf_in = String::new();
        io::stdin().read_to_string(&mut buf_in)
            .expect("Failed to read from stdin. Make sure you piped in valid string data!");
        tweet(&buf_in, &key, &token);
    }

    // delete a tweet
    else if let Some(command) = matches.subcommand_matches("del") {
        let num = command.value_of("id")
            .expect("parse of command line options failed.");
        let num_int = num.parse::<u64>()
            .expect("Please enter a positive whole number");
        delete_tweet(num_int, &key, &token);
    }

    // reply to a tweet
    else if let Some(command) = matches.subcommand_matches("reply") {
        let send_str = command.value_of("words")
            .expect("Could not parse user input. Please check the string is correctly formatted");
        let num = command.value_of("id")
            .expect("parse of command line options failed."); // FIXME user the better message
        let num_int = num.parse::<u64>()
            .expect("Please enter a positive whole number");
        reply(send_str, num_int, &key, &token);
    }

    // retweet a tweet
    else if let Some(command) = matches.subcommand_matches("rt") {
        let num = command.value_of("id")
            .expect("parse of command line options failed.");
        let num_int = num.parse::<u64>()
            .expect("Please enter a positive whole number");
        retweet(num_int, &key, &token);
    }

    // unretweet a tweet
    else if let Some(command) = matches.subcommand_matches("urt") {
        let num = command.value_of("id")
            .expect("parse of command line options failed.");
        let num_int = num.parse::<u64>()
            .expect("Please enter a positive whole number");
        unretweet(num_int, &key, &token);
    }

    // favorite a tweet
    else if let Some(command) = matches.subcommand_matches("fav") {
        let num = command.value_of("id")
            .expect("parse of command line options failed.");
        let num_int = num.parse::<u64>()
            .expect("Please enter a positive whole number");
        favorite_tweet(num_int, &key, &token);
    }

    // unfavorite a tweet
    else if let Some(command) = matches.subcommand_matches("ufav") {
        let num = command.value_of("id")
            .expect("parse of command line options failed.");
        let num_int = num.parse::<u64>()
            .expect("Please enter a positive whole number");
        unfavorite_tweet(num_int, &key, &token);
    }

    // print raw bytes from user's profile; useful for debugging
    else if matches.subcommand_matches("raw").is_some() {
        profile_raw(&key, &token);
    }

    // entered an invalid subcommand
    else {
        println!("No command entered, or command failed to parse. Check tw --help if you need help :)");
    }
}
