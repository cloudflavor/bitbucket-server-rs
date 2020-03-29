extern crate bitbucket;
extern crate tokio;

use bitbucket::client::Client;
use bitbucket::repos::Repositories as repos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        "my_token".to_string(),
        false,
        false,
        "http://bitbucket.company.com".to_string(),
    );
    let repos = repos::build_from_client(client);
    Ok(())
}
