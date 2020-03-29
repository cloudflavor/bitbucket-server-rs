use crate::client::Client;
use crate::prelude::{Repository, Result};

#[derive(Serialize, Deserialize, Default)]
pub struct WebHook {
    endpoint: String,
    client: Client,
}

impl WebHook {
    pub fn new(client: Client) -> WebHook {
        let endpoint = format!("{}/{}", &client.api_url, "pull-requests");
        WebHook { endpoint, client }
    }

    pub fn with_repo(self, _repo: Repository) -> Result<Self> {
        Ok(self)
    }

    pub async fn get(self, _webhook: &str) -> Result<Self> {
        Ok(self)
    }

    pub async fn list(self) -> Result<Self> {
        Ok(self)
    }

    pub async fn delete(self, _webhook: &str) -> Result<()> {
        Ok(())
    }

    pub async fn create(self, _webhook: &str) -> Result<()> {
        Ok(())
    }

    pub async fn update(self, _webhook: &str) -> Result<()> {
        Ok(())
    }
}
