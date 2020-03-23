extern crate tokio;

extern crate bitbucket;

use bitbucket::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_client = Client::new("test", false, false, "http://bitbucket.company.com");
    Ok(())
}
