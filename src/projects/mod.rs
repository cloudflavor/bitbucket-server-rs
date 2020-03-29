pub struct Projects {
    endpoint: String,
}

impl Projects {
    pub fn new() -> Projects {
        Projects {
            endpoint: "/projects".to_string(),
        }
    }
}
