//! Client represents a bitbucket server client.

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub token: String,
    pub disable_ssl: bool,
    pub skip_ssl_verification: bool,
    pub api_url: String,
}

impl Client {
    pub fn new(
        token: String,
        skip_ssl_verification: bool,
        disable_ssl: bool,
        api_url: String,
    ) -> Self {
        Self {
            token,
            skip_ssl_verification,
            disable_ssl,
            api_url,
        }
    }
    pub async fn projects(&self) -> Project {
        Project::new()
    }

    pub async fn repositories(&self) -> Repository {
        Repository::new()
    }
}

impl Default for Client {
    fn default() -> Client {
        Client {
            api_url: "".to_string(),
            disable_ssl: false,
            skip_ssl_verification: false,
            token: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_client() {}
}
