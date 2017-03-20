#[macro_use] extern crate clap;

extern crate oauth_client_fix as oauth_client;
extern crate tweet;
extern crate nom;

use tweet::*;
use clap::App;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};

fn main() {
    // command-line parser
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).get_matches();
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
    // read api keys
    let mut file = File::open(path)
        .expect("File could not be read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("File read failed");
    let (key, token) = get_credentials(&contents);
    // get user timeline
    if let Some(command) = matches.subcommand_matches("user") {
        if let Some(user) = command.value_of("screen_name") {
            if let Some(num) = command.value_of("count") {
                let num_int = num.parse::<u8>()
                    .expect("Please enter a positive whole number");
                print_profile(user, num_int, key, token);
            }
            else {
                // default is to fetch 8 tweets from the profile
                print_profile(user, 8, key, token);
            }
        }
        else {
            if let Some(num) = command.value_of("count") {
                let num_int = num.parse::<u8>()
                    .expect("Please enter a positive whole number");
                print_profile("", num_int, key, token);
            }
            else {
                // this will return the user's own profile
                print_profile("", 8, key, token); 
            }
        }
    }
    // view timeline
    else if let Some(command) = matches.subcommand_matches("view") {
        if let Some(num) = command.value_of("count") {
            let num_int = num.parse::<u8>()
                .expect("Please enter a positive whole number");
            print_timeline(num_int, key, token);
        }
        else {
            // default is to fetch 15 tweets from the user's timeline
            print_timeline(15, key, token);
        }
    }
    // send a tweet
    else if let Some(command) = matches.subcommand_matches("send") {
        let send_str = command.value_of("words")
            .expect("Could not parse user input. Please check the string is correctly formatted");
        tweet(send_str, key, token);
    }
    else {
        // entered an invalid subcommand
        println!("No command entered, or command failed to parse. Check tweet --help if you need help :)");
    }
}
