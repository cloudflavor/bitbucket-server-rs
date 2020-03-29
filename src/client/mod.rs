//! Client represents a bitbucket server client.

use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Client {
    pub token: String,
    pub disable_ssl: bool,
    pub skip_ssl_verification: bool,
    pub api_url: String,
}

impl Client {
    pub fn new(
        token: String,
        api_url: String,
        skip_ssl_verification: bool,
        disable_ssl: bool,
    ) -> Self {
        Self {
            token,
            api_url,
            skip_ssl_verification,
            disable_ssl,
        }
    }
    pub fn projects(self) -> Project {
        Project::new(self)
    }

    pub fn repositories(self) -> Repository {
        Repository::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_client() {}
}
