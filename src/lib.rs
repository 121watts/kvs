pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn get(&self, _: String) -> Option<String> {
        return Some("howdy".to_string());
    }

    pub fn set(&self, _: String, __: String) {}

    pub fn remove(&self, _: String) {}
}
