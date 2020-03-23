pub struct Client<'a> {
    pub token: &'a str,
    pub disable_ssl: bool,
    pub skip_ssl_verification: bool,
    pub api_url: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(
        token: &'a str,
        skip_ssl_verification: bool,
        disable_ssl: bool,
        api_url: &'a str,
    ) -> Result<Self, ()> {
        Ok(Self {
            token: token,
            skip_ssl_verification: skip_ssl_verification,
            disable_ssl: disable_ssl,
            api_url: api_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    #[test]
    fn test_new_client() {
        let test_client = Client::new("123123", false, false, "http://stash.company.com");
    }
}
