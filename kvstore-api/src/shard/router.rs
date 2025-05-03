use super::config::ShardConfig;

#[derive(Clone)]
pub struct ShardRouter {
    pub my_id: usize,
    pub shards: Vec<ShardConfig>,
}

impl ShardRouter {
    pub fn new(my_id: usize, shards: Vec<ShardConfig>) -> Self {
        Self { my_id, shards }
    }

    /// Devuelve el shard correspondiente a una clave
    pub fn shard_for_key(&self, key: &str) -> &ShardConfig {
        self.shards.iter()
            .find(|shard| key >= shard.range_start.as_str() && key <= shard.range_end.as_str())
            .expect("No shard found for key")
    }

    /// ¿La clave pertenece a este nodo?
    pub fn is_local(&self, key: &str) -> bool {
        self.shard_for_key(key).id == self.my_id
    }

    /// Devuelve la dirección IP del nodo responsable de una clave
    pub fn address_for_key(&self, key: &str) -> String {
        self.shard_for_key(key).addr.clone()
    }
}
