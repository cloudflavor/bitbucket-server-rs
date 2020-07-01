use hyper::client::Client;

#[derive(Default, Debug)]
pub(crate) struct Builder {
    client: Client,
}

impl Builder {
    pub fn new(ssl_enabled: bool) -> Self {
        let client = Client::new();
        match ssl_enabled {
            true => println!("true"),
            false => println!("false"),
        }
        Builder { client }
    }
}

#[cfg(test)]
mod tests {
    use hyper::client::Client;

    use crate::builder::Builder;

    #[tokio::test]
    async fn test_new_builder() {
        let client = Client::new();
        let builder = Builder::new(false);
    }
}
