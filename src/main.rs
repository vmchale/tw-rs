#[macro_use] extern crate clap;

extern crate oauth_client;
extern crate libclit;
extern crate nom;

use libclit::*;
use clap::App;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};

fn main() {
    //command-line parser
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
            let mut path_in = std::env::home_dir().expect("");
            path_in.push(".cred");
            path_in
        };
    //read api keys
    let mut file = File::open(path)
        .expect("File could not be read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("File read failed");
    let (key, token) = get_credentials(&contents);
    if let Some(command) = matches.subcommand_matches("user") {
        if let Some(user) = command.value_of("screen_name") {
            print_profile(user, key, token) // does the as_bytes screw us up??
        }
        else {
        print_profile("hung_pope", key, token)
        }
    }
    else if let Some(command) = matches.subcommand_matches("view") {
        if let Some(num) = command.value_of("count") {
            let num_int = num.parse::<u8>()
                .expect("Please enter a positive whole number");
            print_timeline(num_int, key, token);
        }
        else {
            let num_int = 15;
            print_timeline(num_int, key, token);
        }
    }
    else if let Some(command) = matches.subcommand_matches("send") {
        //let args: Vec<String> = env::args().collect();
        let send_str = command.value_of("words")
            .expect("Could not parse user input. Please check the string is correctly formatted");
        tweet(send_str, key, token);
    }
    else {
        println!("No command entered, or command failed to parse. Check tweet --help if you need help :)");
    }
}
