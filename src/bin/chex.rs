extern crate serde;
use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
  up: u8,
  down: u8,
}

fn main() -> std::io::Result<()> {
  let a = Move { up: 1, down: 10 };

  // JSON to and from file
  let file = File::create("move.json")?;

  let serialized_json = serde_json::to_writer_pretty(file, &a).unwrap();

  let file = File::open("move.json")?;
  let reader = BufReader::new(file);
  let deserialized_json: Move = serde_json::from_reader(reader).unwrap();

  println!("serialized json = {:?}", serialized_json);
  println!("deserialized = {:?}", deserialized_json);

  let serialized_vec = bincode::serialize(&a).unwrap();
  let deserialized_vec: Move = bincode::deserialize(&serialized_vec).unwrap();

  println!("serialized Vec<u8> = {:?}", serialized_vec);
  println!("deserialized Vec<u8> = {:?}", deserialized_vec);

  let config = PrettyConfig::new();
  let sr = to_string_pretty(&a, config).unwrap();
  let dsr: Move = from_str(&sr).unwrap();

  println!("serialized ron = {:?}", sr);
  println!("deserialized ron = {:?}", dsr);

  Ok(())

  // let serialized_json = serde_json::to_string(&a).unwrap();

  // let deserialized_json: Move = serde_json::from_str(&serialized_json).unwrap();
  // println!("deserialized = {:?}", deserialized_json);
}
