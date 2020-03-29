use crate::client::Client;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Repository {
    endpoint: String,
    client: Client,
}

impl Repository {
    pub fn new(client: Client) -> Repository {
        let endpoint = format!("{}/{}", &client.api_url, "repos");
        Repository { endpoint, client }
    }

    pub fn with_project(self, _project: Project) -> Self {
        self
    }

    pub fn get(self, _repository: &str) -> Result<Self> {
        Ok(self)
    }

    pub fn comments(self) -> Comment {
        Comment::new(self.client)
    }

    pub fn create(self, _repository: &str) -> Result<()> {
        Ok(())
    }

    pub fn delete(self, _repository: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
