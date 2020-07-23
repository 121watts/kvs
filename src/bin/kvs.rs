#[macro_use]
extern crate clap;
use clap::{load_yaml, App};
use std::process;
use kvs::{Result};

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let config = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .name(crate_name!());

    let matches = config.get_matches();

    match matches.subcommand() {
        ("get", Some(sub_input)) => err_unimplemented(sub_input),
        ("set", Some(sub_input)) => err_unimplemented(sub_input),
        ("rm", Some(sub_input)) => err_unimplemented(sub_input),
        _ => println!("no command matches"),
    }

    fn err_unimplemented(_: &clap::ArgMatches) {
        eprintln!("unimplemented");
        process::exit(1);
    }

    Ok(())
}
