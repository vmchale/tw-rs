#[macro_use] extern crate clap;

extern crate oauth_client;
extern crate libclit;
extern crate nom;

use libclit::*;
use clap::App;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env::home_dir;
use std::path::{Path, PathBuf};

fn main() {
    //parse command-line options
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).get_matches();
    //read api keys
    let path_read = matches.value_of("credentials");
    let mut path = 
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
    else {
        println!("No command entered, or command failed to parse. Check tweet --help if you need help :)");
    }
}
