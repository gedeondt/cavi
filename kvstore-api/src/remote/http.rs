use super::RemoteNodeClient;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct HttpNodeClient {
    client: Client,
}

impl HttpNodeClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl RemoteNodeClient for HttpNodeClient {
    async fn forward_get(&self, key: &str, remote_addr: &str) -> Result<Option<String>, String> {
        let url = format!("http://{}/kv/{}", remote_addr, key);
        let res = self.client.get(&url).send().await.map_err(|e| e.to_string())?;

        match res.status().as_u16() {
            200 => {
                let val: String = res.json().await.map_err(|e| e.to_string())?;
                Ok(Some(val))
            }
            404 => Ok(None),
            _ => Err(format!("Unexpected status: {}", res.status())),
        }
    }

    async fn forward_set(&self, key: &str, value: &str, remote_addr: &str) -> Result<(), String> {
        let url = format!("http://{}/kv/{}", remote_addr, key);
        let res = self
            .client
            .put(&url)
            .json(&json!({ "value": value }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Set failed: {}", res.status()))
        }
    }

    async fn forward_delete(&self, key: &str, remote_addr: &str) -> Result<(), String> {
        let url = format!("http://{}/kv/{}", remote_addr, key);
        let res = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Delete failed: {}", res.status()))
        }
    }
}
