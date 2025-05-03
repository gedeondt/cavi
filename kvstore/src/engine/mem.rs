use std::collections::BTreeMap;
use crate::kv::KvStore;
use crate::error::KvResult;

pub struct MemStore {
    map: BTreeMap<String, String>,
}

impl MemStore {
    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }
}

impl KvStore for MemStore {
    fn get(&self, key: &str) -> KvResult<Option<String>> {
        Ok(self.map.get(key).cloned())
    }

    fn search_by_prefix(&self, prefix: &str) -> KvResult<Vec<(String, String)>> {
        let range = self.map.range(prefix.to_string()..);
        let results = range
            .take_while(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Ok(results)
    }

    fn set(&mut self, key: String, value: String) -> KvResult<()> {
        self.map.insert(key, value);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> KvResult<()> {
        self.map.remove(key);
        Ok(())
    }
}
