use crate::client::Client;
use crate::prelude::{Repository, Result};

#[derive(Serialize, Deserialize, Default)]
pub struct Comment {
    endpoint: String,
    client: Client,
}

impl Comment {
    pub fn new(client: Client) -> Comment {
        let endpoint = format!("{}/{}", client.api_url, "comments");
        Comment { endpoint, client }
    }
    pub fn with_repo(self, _repo: Repository) -> Result<Self> {
        Ok(self)
    }
    pub fn get(self, _comment: &str) -> Result<Self> {
        Ok(self)
    }

    pub fn list(self) -> Result<Vec<Self>> {
        Ok(vec![self])
    }

    pub fn delete(self, _comment: &str) -> Result<Self> {
        Ok(self)
    }

    pub fn create(self, _comment: &str) -> Result<Self> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {}
