#[macro_use]
extern crate clap;
use anyhow::anyhow;
use clap::{load_yaml, App};
use kvs::{KvStore, Result};
use std::env;

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            println!("{:?}", err);
            1
        }
    });
}

fn run_app() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let config = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .name(crate_name!());

    let matches = config.get_matches();

    let m = match matches.subcommand() {
        ("get", Some(sub_input)) => get(&sub_input),
        ("set", Some(sub_input)) => set(&sub_input),
        ("rm", Some(sub_input)) => remove(&sub_input),
        _ => Err(anyhow!("no command matches")),
    };

    m
}

fn set(set_args: &clap::ArgMatches) -> Result<()> {
    let key = set_args.value_of("key").unwrap();
    let value = set_args.value_of("value").unwrap();
    let path = env::current_dir().expect("Could not open logs");
    let mut store = KvStore::open(&path).expect("Could not open KvStore");

    let value = store.set(key.to_owned(), value.to_owned())?;
    println!("Value set: {:?}", value);

    Ok(())
}

fn get(get_arg: &clap::ArgMatches) -> Result<()> {
    let key = get_arg.value_of("key").unwrap();
    let path = env::current_dir().expect("Could not open logs");
    let mut store = KvStore::open(&path).expect("Could not open KvStore");

    match store.get(key.to_owned())? {
        Some(val) => println!("{}", val),
        None => println!("Key not found"),
    }

    Ok(())
}

fn remove(rm_arg: &clap::ArgMatches) -> Result<()> {
    let key = rm_arg.value_of("key").unwrap();
    let path = env::current_dir().expect("Could not open logs");
    let mut store = KvStore::open(&path).expect("Could not open KvStore");

    store.remove(key.to_owned())?;

    Ok(())
}
