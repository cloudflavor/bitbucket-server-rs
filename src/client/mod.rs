//! Client represents a bitbucket server client.

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
}

#[cfg(test)]
mod tests {
    use super::Client;
    #[test]
    fn test_new_client() {
        let test_client = Client::new(
            "123123".to_string(),
            false,
            false,
            "http://stash.company.com".to_string(),
        );
    }
}
