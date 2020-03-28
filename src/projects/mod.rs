#[derive(Debug)]
pub struct Repos {
    endpoint: String,
}

impl Repos {
    pub fn new() -> Repos {
        Repos {
            endpoint: "/repos".to_string(),
        }
    }
}
