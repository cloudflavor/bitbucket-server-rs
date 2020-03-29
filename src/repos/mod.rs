pub mod comments;

pub use comments::Comments;

pub struct Repositories<Client> {
    client: Client,
    endpoint: String,
}

impl<Client> Repositories<Client> {
    pub fn build_from_client(client: Client) -> Self {
        Repositories {
            client,
            endpoint: "/repos".to_string(),
        }
    }
    pub fn get_repos(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Repositories as Repos;
    use crate::client::Client;

    #[test]
    fn test_new_repos_builder() {
        let c = Client::new(
            "token".to_string(),
            true,
            false,
            "api.company.com".to_string(),
        );
        let r = Repos::build_from_client(c);
    }
}
