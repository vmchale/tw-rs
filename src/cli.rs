extern crate clap;

use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let yaml = load_yaml!("options-en.yml");
    App::from_yaml(yaml);
}
