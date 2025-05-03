pub mod engine;
pub mod kv;
pub mod error;

pub use kv::{KvStore, KvResult};

#[cfg(test)]
mod tests {
    use super::engine::mem::MemStore;
    use super::kv::KvStore;

    #[test]
    fn test_store() {
        let mut store = MemStore::new();
        store.set("key1".into(), "value1".into()).unwrap();
        assert_eq!(store.get("key1").unwrap(), Some("value1".into()));
        store.delete("key1").unwrap();
        assert_eq!(store.get("key1").unwrap(), None);
    }

    #[test]
    fn test_search_by_prefix() {
        let mut store = MemStore::new();
        store.set("apple".into(), "fruit".into()).unwrap();
        store.set("apricot".into(), "fruit".into()).unwrap();
        store.set("banana".into(), "fruit".into()).unwrap();
    
        let results = store.search_by_prefix("ap").unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|(k, _)| k == "apple"));
        assert!(results.iter().any(|(k, _)| k == "apricot"));
    }
}
