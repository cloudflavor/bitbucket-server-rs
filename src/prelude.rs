//! Prelude exports the main library types for import convenience.
pub use crate::resources::comments::Comment;
pub use crate::resources::projects::Project;
pub use crate::resources::pullrequests::PullRequest;
pub use crate::resources::repositories::Repository;
pub use crate::resources::webhooks::WebHook;

pub(crate) type Result<T> = std::result::Result<T, std::io::Error>;
