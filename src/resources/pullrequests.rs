use crate::client::Client;
use crate::prelude::Result;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PullRequest {
    endpoint: String,
    client: Client,
}

impl PullRequest {
    pub fn new(client: Client) -> PullRequest {
        PullRequest {
            endpoint: "pull-request".to_string(),
            client,
        }
    }
    pub async fn get(self, _pullrequest: &str) -> Result<Self> {
        Ok(self)
    }
    pub async fn list(self, _pullrequest: &str) -> Result<Self> {
        Ok(self)
    }
    pub async fn delete(self, _pullrequest: &str) -> Result<Self> {
        Ok(self)
    }
    pub async fn create(self, _pullrequest: &str) -> Result<Self> {
        Ok(self)
    }
    pub async fn update(self, _pullrequest: &str) -> Result<Self> {
        Ok(self)
    }
}
