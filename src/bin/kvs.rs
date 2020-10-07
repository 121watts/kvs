#[macro_use]
extern crate clap;
use clap::{load_yaml, App};
use kvs::{KvStore, Result};
use std::env;
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
        ("rm", Some(sub_input)) => remove(&sub_input),
        _ => println!("no command matches"),
    }

    Ok(())
}

fn set(set_args: &clap::ArgMatches) {
    let key = set_args.value_of("key").unwrap();
    let value = set_args.value_of("value").unwrap();
    let path = env::current_dir().expect("Could not open logs");
    let mut store = KvStore::open(&path).expect("Could not open KvStore");

    match store.set(key.to_owned(), value.to_owned()) {
        Ok(value) => println!("Value set: {:?}", value),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn get(get_arg: &clap::ArgMatches) {
    let key = get_arg.value_of("key").unwrap();
    let path = env::current_dir().expect("Could not open logs");
    let mut store = KvStore::open(&path).expect("Could not open KvStore");
    store.make_index(&path);

    match store.get(key.to_owned()) {
        Ok(ok) => match ok {
            Some(val) => println!("{:?}", val),
            _ => println!("idk"),
        },
        Err(err) => println!("Error: {:?}", err),
    }
}

fn remove(rm_arg: &clap::ArgMatches) {
    let key = rm_arg.value_of("key").unwrap();
    let path = env::current_dir().expect("Could not open logs");
    let mut store = KvStore::open(&path).expect("Could not open KvStore");
    store.make_index(&path);

    match store.remove(key.to_owned()) {
        Ok(key) => println!("Removed value for key: {:?}", key),
        Err(err) => println!("Error: {:?}", err),
    }
}
