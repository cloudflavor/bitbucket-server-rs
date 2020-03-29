# bitbucket-rs

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Rust][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/bitbucket
[crates-url]: https://crates.io/crates/bitbucket
[docs-badge]: https://docs.rs/bitbucket/badge.svg
[docs-url]: https://docs.rs/bitbucket
[actions-badge]: https://github.com/cloudflavor/bitbucket-rs/workflows/Rust/badge.svg?branch=master&event=push
[actions-url]: https://github.com/cloudflavor/bitbucket-rs/actions?query=branch%3Amaster+

NOTE: _work-in-progress_ async rust bindings for bitbucket server.
This is published so that the name is reserved for use on crates.io.

## Examples

```rust
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
    let proj = Project::new(client).get("my_project").await?;
    println!("{:?}", proj);
    Ok(())
}

```

Build and read the documentation locally.

```bash
$ cargo doc --open -p bitbucket --no-deps
Documenting bitbucket v0.1.0-alpha (rust/bitbucket-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
```
