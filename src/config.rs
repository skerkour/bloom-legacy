use std::env;
use serde::{Serialize, Deserialize};
use dotenv;

pub fn init() -> Config {
    dotenv::dotenv().ok();

    // set default env vars
    if env::var("PORT").is_err() {
        env::set_var("PORT", "8000")
    }

    return Config{
        port: get_env("PORT"),
        database_url: get_env("DATABASE_URL"),
    };
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    port: String,
    database_url: String,
}

impl Config {
    pub fn port(&self) -> String {
        return self.port.clone();
    }

    pub fn database_url(&self) -> String {
        return self.database_url.clone();
    }
}

//     // 	// Port is the port to listen to
// 	// Port string
// 	// // DatabaseURL is the database's URL to connect to
// 	// DatabaseURL string
// 	// // GoEnv represents where the API server is ran (development, test, staging, production)
// 	// GoEnv string
// 	// // SendgridAPIKey is required to send emails
// 	// SendgridAPIKey string

// 	// // GitHub oauth2 token to retreive contributors
// 	// GitHubToken string

// 	// AWSSecretAccessKey string
// 	// AWSAccessKeyID     string
// 	// AWSRegion          string
// 	// AWSS3Bucket        string

// 	// AssetsPath string

// 	// AWSSQSApiToBitflow string
// 	// AWSSQSBitflowToApi string
// 	// BitflowSecret string

// 	// AWSSQSApiToPhaser string
// 	// AWSSQSPhaserToApi string
// 	// PhaserSecret string

// 	// WWWHost string

// 	// SentryURL string
// }

pub fn get_env(var: &str) -> String {
    env::var(var).expect(&format!("Missing environment variable: {}", var)).to_string()
}
