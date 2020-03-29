use crate::client::Client;
use crate::prelude::Result;

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

    pub fn get(self, _webhook: &str) -> Result<Self> {
        Ok(self)
    }
}
