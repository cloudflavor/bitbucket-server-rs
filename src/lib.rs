//! bitbucket-rs
//!
//! Bitbucket is an async library that provides Rust bindings to
//! Atlassian Bitbucket server (formerly known as Stash).

#[macro_use]
extern crate serde;

pub mod client;
pub mod resources;

pub use resources::comments;
pub use resources::projects;
pub use resources::repositories;

pub mod prelude {
    pub use crate::resources::comments::Comment;
    pub use crate::resources::projects::Project;
    pub use crate::resources::repositories::Repository;
}
