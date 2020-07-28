#![deny(missing_docs)]
//! kvs is a command line key value store
use anyhow::anyhow;
// use std::collections::HashMap;
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

/// Result type for the KvStore
pub type Result<T> = anyhow::Result<T>;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
  Get(String),
  Set(String, String),
  Remove(String, String),
}

/// KvStore struct definition
pub struct KvStore {
  file: File,
  store: HashMap<String, String>,
}

impl KvStore {
  /// Creates a new KvStore
  /// # Examples
  /// ```
  /// let store = KvStore::new();
  /// ```
  pub fn new(file: File) -> Self {
    KvStore {
      file,
      store: HashMap::new(),
    }
  }

  /// Opens the command log
  pub fn open(path: &Path) -> Result<KvStore> {
    let log_file = path.join("commands.log");
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .append(true)
      .create(true)
      .open(log_file);

    // TODO: store contents in memory
    match file {
      Ok(file) => Ok(KvStore::new(file)),
      Err(_) => Err(anyhow!("Could not open file")),
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
    let value = self.store.get(&key);

    match value {
      Some(value) => Ok(Some(value.to_string())),
      None => Err(anyhow!("Key not found")),
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
  pub fn remove(&mut self, key: String) -> Result<String> {
    let value = self.store.remove(&key);

    match value {
      Some(value) => Ok(value.to_string()),
      None => Err(anyhow!("Key not found")),
    }
  }
}
