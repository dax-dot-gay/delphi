use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub admin_user: Option<(String, String)>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabaseConfig {
    Uri {
        uri: String
    },
    Args {
        host: String,
        port: u16,
        username: String,
        password: String
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::Uri { uri: "mongodb://root:root@db:27017/".to_string() }
    }
}

impl DatabaseConfig {
    pub fn uri(&self) -> String {
        match self {
            DatabaseConfig::Uri { uri } => uri.clone(),
            DatabaseConfig::Args { host, port, username, password } => format!("mongodb://{username}:{password}@{host}:{port}/"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub auth: AuthConfig,

    #[serde(default)]
    pub database: DatabaseConfig
}