use async_trait::async_trait;

pub mod http;
pub use http::HttpNodeClient;


#[async_trait]
pub trait RemoteNodeClient: Send + Sync {
    async fn forward_get(&self, key: &str, remote_addr: &str) -> Result<Option<String>, String>;
    async fn forward_set(&self, key: &str, value: &str, remote_addr: &str) -> Result<(), String>;
    async fn forward_delete(&self, key: &str, remote_addr: &str) -> Result<(), String>;
}
