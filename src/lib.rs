//! bitbucket-rs
//!
//! Bitbucket is an async library that provides Rust bindings for
//! the [Core API Atlassian Bitbucket server](https://docs.atlassian.com/bitbucket-server/rest/7.1.0/bitbucket-rest.html)
//!  (formerly known as Stash).  
//!
//!  For now only the main resources are covered in this library.
//!
//!  # Examples:
//!
//! ```rust
//! extern crate bitbucket;
//! use bitbucket::prelude::*;
//! use bitbucket::client::Client;
//!
//!
//! extern crate bitbucket;
//! extern crate tokio;

//! use bitbucket::client::Client;
//! use bitbucket::prelude::*;

//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(
//!         "my_token".to_string(),
//!         "http://bitbucket.company.com".to_string(),
//!         false,
//!         false,
//!     );
//!     let proj = Project::new(client).get("my_project").await?;
//!     println!("{:?}", proj);
//!     Ok(())
//! }
//! ```

#[macro_use]
extern crate serde;

pub(crate) mod builder;
pub mod client;
pub mod prelude;
pub mod resources;

pub use resources::comments;
pub use resources::projects;
pub use resources::pullrequests;
pub use resources::repositories;
pub use resources::webhooks;
