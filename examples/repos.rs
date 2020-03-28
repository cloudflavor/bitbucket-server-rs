extern crate tokio;

extern crate bitbucket;

use bitbucket::client::Client;
use bitbucket::projects::Repos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_client = Client::new(
        "test".to_string(),
        false,
        false,
        "http://bitbucket.company.com".to_string(),
    );
    let projects = Repos::new();
    let projects = local_client.build(projects);
    Ok(())
}
