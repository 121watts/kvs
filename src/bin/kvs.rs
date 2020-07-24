#[macro_use]
extern crate clap;
use clap::{load_yaml, App};
use kvs::{KvStore, Result};
use std::process;

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let config = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .name(crate_name!());

    let matches = config.get_matches();

    match matches.subcommand() {
        ("get", Some(sub_input)) => get(&sub_input),
        ("set", Some(sub_input)) => set(&sub_input),
        ("rm", Some(sub_input)) => err_unimplemented(sub_input),
        _ => println!("no command matches"),
    }

    fn err_unimplemented(_: &clap::ArgMatches) {
        eprintln!("unimplemented");
        process::exit(1);
    }

    Ok(())
}

fn set(_: &clap::ArgMatches) {
    // println!("sum match: {:?}", m)
}

fn get(get_arg: &clap::ArgMatches) {
    let key = get_arg.value_of("key").unwrap();
    let mut store = KvStore::new();

    match store.get(key.to_owned()) {
        Ok(ok) => match ok {
            Some(val) => println!("{:?}", val),
            _ => println!("idk"),
        },
        Err(e) => println!("{:?}", e),
    }
}
