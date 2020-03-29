extern crate bitbucket;
extern crate tokio;

use bitbucket::client::Client;
use bitbucket::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        "my_token".to_string(),
        "http://bitbucket.company.com".to_string(),
        false,
        false,
    );
    let proj = Project::new(client);
    proj.get("my_repo").await.unwrap();
    Ok(())
}
