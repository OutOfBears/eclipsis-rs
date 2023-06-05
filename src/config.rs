use crate::apis::configuration::{ApiKey, Configuration};

pub struct ConfigurationBuilder {
    configuration: Configuration,
}

impl ConfigurationBuilder {
    pub fn new() -> ConfigurationBuilder {
        ConfigurationBuilder {
            configuration: Configuration::new(),
        }
    }

    pub fn user_agent(&mut self, user_agent: String) -> &mut Self {
        self.configuration.user_agent = Some(user_agent);
        self
    }

    pub fn api_key(&mut self, api_key: String) -> &mut Self {
        self.configuration.api_key = Some(ApiKey {
            prefix: None,
            key: api_key,
        });

        self
    }

    pub fn client(&mut self, client: reqwest::Client) -> &mut Self {
        self.configuration.client = reqwest_middleware::ClientBuilder::new(client).build();
        self
    }

    pub fn build(&self) -> Configuration {
        self.configuration.clone()
    }
}
