use crate::client::Client;
use crate::prelude::Result;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Project {
    endpoint: String,
    client: Client,
}

impl Project {
    pub fn new(client: Client) -> Project {
        let endpoint = format!("{}/{}", &client.api_url, "projects");
        Project { endpoint, client }
    }
    pub async fn get(self, _project: &str) -> Result<Self> {
        Ok(self)
    }

    pub async fn list(self) -> Result<Vec<Self>> {
        Ok(vec![self])
    }

    pub async fn delete(self, _project: &str) -> Result<()> {
        Ok(())
    }

    pub async fn create(self, _project: &str) -> Result<()> {
        Ok(())
    }

    pub async fn update(self, _project: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Project;
    use crate::client::Client;

    #[tokio::test]
    async fn test_get_project() {
        let c = Client::new("token".to_string(), "api".to_string(), false);
        let p = Project::new(c);
        p.get("my_project").await.unwrap();
    }
}
