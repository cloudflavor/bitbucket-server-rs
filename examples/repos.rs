extern crate bitbucket;
extern crate tokio;

use bitbucket::client::Client;
use bitbucket::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _client = Client::new(
        "my_token".to_string(),
        false,
        false,
        "http://bitbucket.company.com".to_string(),
    );
    let _proj = Project::new();
    Ok(())
}
