use config::Config;
use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;

pub static CONFIGURATION: Lazy<ServerConfiguration> = Lazy::new(ServerConfiguration::load);

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde(default = "ServerConfiguration::default_host")]
    pub host: String,
    #[serde(default = "ServerConfiguration::default_port")]
    pub port: u32,
    #[serde(default = "ServerConfiguration::default_hmac_key")]
    pub hmac_key: String,
}

impl ServerConfiguration {
    pub fn load() -> Self {
        let path = std::path::PathBuf::from("Settings.toml");
        if !path.exists() {
            let server_conf = ServerConfiguration::default();
            let conf = toml::to_string(&server_conf).unwrap();
            std::fs::write(path, conf).unwrap();
        }
        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .add_source(config::Environment::with_prefix("BOODSCHAPPIES"))
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()
    }

    pub fn get_hmac_key(&self) -> Vec<u8> {
        self.hmac_key.as_bytes().to_vec()
    }

    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    fn default_port() -> u32 {
        30301
    }

    fn default_hmac_key() -> String {
        "CHANGE_ME".to_string()
    }
}

impl std::default::Default for ServerConfiguration {
    fn default() -> Self {
        ServerConfiguration {
            host: ServerConfiguration::default_host(),
            port: ServerConfiguration::default_port(),
            hmac_key: ServerConfiguration::default_hmac_key(),
        }
    }
}
