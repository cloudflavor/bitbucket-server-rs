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
    );
    let proj = Project::new(client).get("my_project").await?;
    println!("{:?}", proj);

    let repo = Repository::new(client)
        .build("my_repo")
        .with_project(proj)
        .comments()
        .list()
        .await?;

    let prs = my_repo.pull_request().list().await?;
    Ok(())

    let client = Client::New()
    client::get().project("test")
    client::repo("test").with_project("project")
}
