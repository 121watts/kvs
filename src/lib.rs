#![deny(missing_docs)]
//! kvs is a command line key value store
use anyhow::anyhow;
// use std::collections::HashMap;
use filepath::FilePath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::{Path, PathBuf};

/// Result type for the KvStore
pub type Result<T> = anyhow::Result<T>;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
  Get(String),
  Set(String, String),
  Remove(String),
}

/// KvStore struct definition
pub struct KvStore {
  file: File,
  store: HashMap<String, usize>,
  reader: BufReader<File>,
}

impl KvStore {
  /// Creates a new KvStore
  /// # Examples
  /// ```
  /// let store = KvStore::new();
  /// ```
  pub fn new(file: File, reader: BufReader<File>) -> Self {
    let store = HashMap::new();

    KvStore {
      file,
      store,
      reader,
    }
  }

  /// Opens the command log
  pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
    let log_file = make_path(&path.into());

    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .append(true)
      .create(true)
      .open(&log_file);

    let reader = make_reader(&log_file);

    match file {
      Ok(file) => Ok(KvStore::new(file, reader)),
      Err(_) => Err(anyhow!("Could not open file")),
    }
  }

  /// Makes the index
  pub fn make_index(&mut self) {
    let path = self.file.path().expect("Could not open file at path");
    let file = OpenOptions::new()
      .read(true)
      .open(path)
      .expect("Could not open file");

    let reader = BufReader::new(file);
    let mut pointer: usize = 0;

    for line in reader.lines() {
      let line = line.unwrap();
      let command: Command = bincode::deserialize(line.as_bytes()).expect("Could not deserialize");

      match command {
        Command::Set(key, _) => {
          self.store.insert(key, pointer);
        }
        Command::Remove(key) => {
          self.store.remove(&key);
        }
        _ => {}
      }

      pointer += (line + "\n").len();
    }
  }

  /// Sets a value for a particular key
  /// # Example
  ///
  /// ```
  /// let store = KvStore::new();
  /// store.set("key1".to_owned(), "value1".to_owned());
  /// ```
  pub fn set(&mut self, key: String, value: String) -> Result<String> {
    let val_clone = value.clone();
    let command = Command::Set(key, value);
    let b_command = bincode::serialize(&command).unwrap();

    self
      .file
      .write_all(&b_command)
      .expect("Could not set value");

    self.file.write_all(b"\n").expect("Could not set value");

    Ok(val_clone)
  }

  /// Gets a value for a particular key.
  /// Returns `None` if a value is not found.
  /// # Example
  ///
  /// ```
  /// let store = KvStore::new();
  /// store.set("key1".to_owned(), "value1".to_owned());
  /// store.get("key1".to_owned());
  /// ```
  pub fn get(&mut self, key: String) -> Result<Option<String>> {
    self.make_index();
    let value = self.store.get(&key);

    match value {
      Some(value) => {
        let pointer = *value as u64;
        self.reader.seek(SeekFrom::Start(pointer))?;

        let mut buf = String::new();

        self.reader.read_line(&mut buf)?;
        let command: Command = bincode::deserialize(buf.as_bytes()).expect("Could not deserialize");

        match command {
          Command::Set(_, value) => Ok(Some(value)),
          _ => {
            let op: Option<String> = None;
            Ok(op)
          }
        }
      }

      None => {
        let op: Option<String> = None;
        Ok(op)
      }
    }
  }

  /// Removes a value at a particular key.
  /// # Examples
  ///
  /// ```
  /// let store = KvStore::new();
  /// store.set("key1".to_owned(), "value1".to_owned());
  /// store.remove("key1".to_owned());
  /// ```
  pub fn remove(&mut self, key: String) -> Result<()> {
    self.make_index();
    let value = self.store.remove(&key);

    match value {
      Some(_) => {
        let command = Command::Remove(key);
        let command = bincode::serialize(&command).unwrap();

        self
          .file
          .write_all(&command)
          .expect("Could not set Command::Remove to log");

        self
          .file
          .write_all(b"\n")
          .expect("Could not set Command::Remove to log");

        Ok(())
      }
      None => Err(anyhow!("Key not found")),
    }
  }
}

// fn make_writer(path: &Path) -> BufWriter<File> {
//   let log_file = path.join("commands.log");
//   let file = OpenOptions::new()
//     .read(true)
//     .write(true)
//     .append(true)
//     .create(true)
//     .open(log_file)
//     .expect("Could not open file");
//
//   BufWriter::new(file)
// }

fn make_reader(path: &Path) -> BufReader<File> {
  let file = OpenOptions::new()
    .read(true)
    .open(path)
    .expect("Could not open file");

  BufReader::new(file)
}

fn make_path(path: &Path) -> PathBuf {
  path.join("commands.log")
}
