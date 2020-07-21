#![deny(missing_docs)]
//! kvs is a command line key value store
use std::collections::HashMap;

/// KvStore struct definition
#[derive(Default)]
pub struct KvStore {
  store: HashMap<String, String>,
}

impl KvStore {
  /// Creates a new KvStore
  /// # Examples
  /// ```
  /// let store = KvStore::new();
  /// ```
  pub fn new() -> Self {
    KvStore {
      store: HashMap::new(),
    }
  }

  /// Sets a value for a particular key
  /// # Example
  ///
  /// ```
  /// let store = KvStore::new();
  /// store.set("key1".to_owned(), "value1".to_owned());
  /// ```
  pub fn set(&mut self, key: String, value: String) {
    self.store.insert(key, value);
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
  pub fn get(&mut self, key: String) -> Option<String> {
    self.store.get(&key).cloned()
  }

  /// Removes a value at a particular key.
  /// # Examples
  ///
  /// ```
  /// let store = KvStore::new();
  /// store.set("key1".to_owned(), "value1".to_owned());
  /// store.remove("key1".to_owned());
  /// ```
  pub fn remove(&mut self, key: String) {
    self.store.remove(&key);
  }
}
