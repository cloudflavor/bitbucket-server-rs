use crate::client::Client;
use crate::prelude::Result;

#[derive(Serialize, Deserialize, Default)]
pub struct PullRequest {
    endpoint: String,
    client: Client,
}

impl PullRequest {
    pub fn new(client: Client) -> PullRequest {
        let endpoint = format!("{}/{}", &client.api_url, "pull-requests");
        PullRequest { endpoint, client }
    }

    pub fn get(self, _pullrequest: &str) -> Result<Self> {
        Ok(self)
    }
}
