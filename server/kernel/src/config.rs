use std::env;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::fs;
use regex::Regex;
use url::Url;
use crate::KernelError;


#[derive(Clone, Debug, Deserialize, Serialize)]
struct ConfigFile {
    rust_env: Environment,
    host: String,
    port: u16,
    smtp: SmtpConfig,
    database: DatabaseConfig,
    aws: AwsConfig,
    s3: S3Config,
    sentry: SentryConfig,
    phaser: PhaserConfig,
    bitflow: BitflowConfig,
    blacklists: BlacklistsConfig,
    stripe: StripeConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub rust_env: Environment,
    pub host: String,
    pub port: u16,
    pub smtp: SmtpConfig,
    pub database: DatabaseConfig,
    pub aws: AwsConfig,
    pub s3: S3Config,
    pub sentry: SentryConfig,
    pub phaser: PhaserConfig,
    pub bitflow: BitflowConfig,
    pub stripe: StripeConfig,
    pub disposable_email_domains: HashSet<String>,
    pub basic_passwords: HashSet<String>,
    pub version: String,
}

impl From<ConfigFile> for Config {
    fn from(config: ConfigFile) -> Self {
        let mut blacklisted_email_domains = HashSet::new();
        let mut blacklisted_passwords = HashSet::new();

        for email_domains_file_path in config.blacklists.email_domains {
            let email_domains_file = File::open(&email_domains_file_path).expect("Error opening blacklist email domains file");

            let blacklisted_email_domain_lines: Vec<String> = BufReader::new(email_domains_file)
                .lines()
                .filter_map(Result::ok)
                .collect();

            blacklisted_email_domain_lines.iter().for_each(|domain| {
                blacklisted_email_domains.insert(domain.to_string());
            });
        }

        for common_passwords_file_path in config.blacklists.passwords {
            let common_passwords_file = File::open(&common_passwords_file_path).expect("Error opening password email domains file");

            let common_passwords_lines: Vec<String> = BufReader::new(common_passwords_file)
                .lines()
                .filter_map(Result::ok)
                .collect();

            common_passwords_lines.iter().for_each(|password| {
                blacklisted_passwords.insert(password.to_string());
            });
        }

        let version = include_str!("../../../VERSION.txt").trim().to_string();

        return Config {
            rust_env: config.rust_env,
            host: config.host,
            port: config.port,
            smtp: config.smtp,
            database: config.database,
            aws: config.aws,
            s3: config.s3,
            sentry: config.sentry,
            phaser: config.phaser,
            bitflow: config.bitflow,
            stripe: config.stripe,
            disposable_email_domains: blacklisted_email_domains,
            basic_passwords: blacklisted_passwords,
            version,
        };
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), KernelError> {
        // phaser and bitflow secret length
        if self.phaser.secret.len() < 64 {
            return Err(KernelError::Validation("Phaser secret length must be at least 64.".to_string()));
        }
        if self.bitflow.secret.len() < 64 {
            return Err(KernelError::Validation("Bitflow secret length must be at least 64.".to_string()));
        }

        // host have scheme
        let parsed_host = Url::parse(&self.host)?;
        if parsed_host.scheme().is_empty() {
            return Err(KernelError::Validation("Host musht have a URL scheme. eg. http://localhost:8080.".to_string()));
        }

        // others are not empty
        if self.database.url.is_empty() {
            return Err(KernelError::Validation("Database URL must not be empty.".to_string()));
        }
        return Ok(());
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Environment {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "production")]
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Test => write!(f, "test"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AwsConfig {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct S3Config {
    pub bucket: String,
    pub base_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhaserConfig {
    pub secret: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitflowConfig {
    pub secret: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmtpConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SentryConfig {
    pub server_url: Option<String>,
    pub webapp_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlacklistsConfig {
    pub email_domains: Vec<String>,
    pub passwords: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StripeConfig {
    pub public_key: String,
    pub secret_key: String,
}


pub fn init() -> Config {
    let config_file_contents = fs::read_to_string("bloom.sane")
        .expect("Error reading bloom.sane");

    let decoded: ConfigFile = sane::from_str(&config_file_contents)
        .expect("Error parsing config file");

    let decoded = replace_env(decoded);

    env::set_var("AWS_ACCESS_KEY_ID", decoded.aws.access_key_id.clone());
    env::set_var("AWS_SECRET_ACCESS_KEY", decoded.aws.secret_access_key.clone());
    env::set_var("AWS_REGION", decoded.aws.region.clone());
    env::set_var("PHASER_SECRET", decoded.phaser.secret.clone());
    env::set_var("BITFLOW_SECRET", decoded.bitflow.secret.clone());

    return decoded.into();
}

fn replace_env(mut config: ConfigFile) -> ConfigFile {
    let re = Regex::new(r"\$\{[A-Z_0-9]*\}").expect("error compiling env regex");
    let patterns : &[_] = &['$', '{', '}'];

    // host
    for match_ in re.find_iter(&config.host.clone()) {
        let match_str = match_.as_str();
        config.host = config.host.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // smpt
    for match_ in re.find_iter(&config.smtp.host.clone()) {
        let match_str = match_.as_str();
        config.smtp.host = config.smtp.host.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }
    for match_ in re.find_iter(&config.smtp.username.clone()) {
        let match_str = match_.as_str();
        config.smtp.username = config.smtp.username.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }
    for match_ in re.find_iter(&config.smtp.password.clone()) {
        let match_str = match_.as_str();
        config.smtp.password = config.smtp.password.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // database
    for match_ in re.find_iter(&config.database.url.clone()) {
        let match_str = match_.as_str();
        config.database.url = config.database.url.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // aws
    for match_ in re.find_iter(&config.aws.access_key_id.clone()) {
        let match_str = match_.as_str();
        config.aws.access_key_id = config.aws.access_key_id.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }
    for match_ in re.find_iter(&config.aws.secret_access_key.clone()) {
        let match_str = match_.as_str();
        config.aws.secret_access_key = config.aws.secret_access_key.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }
    for match_ in re.find_iter(&config.aws.region.clone()) {
        let match_str = match_.as_str();
        config.aws.region = config.aws.region.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // s3
    for match_ in re.find_iter(&config.s3.bucket.clone()) {
        let match_str = match_.as_str();
        config.s3.bucket = config.s3.bucket.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }
    for match_ in re.find_iter(&config.s3.base_url.clone()) {
        let match_str = match_.as_str();
        config.s3.base_url = config.s3.base_url.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // sentry
    if let Some(ref url) = config.sentry.server_url {
        let url = url.clone();
        for match_ in re.find_iter(&url) {
            let match_str = match_.as_str();
            config.sentry.server_url = Some(url.replace(match_str, &get_env(match_str.trim_matches(patterns))));
        }
    }
    if let Some(ref url) = config.sentry.webapp_url {
        let url = url.clone();
        for match_ in re.find_iter(&url) {
            let match_str = match_.as_str();
            config.sentry.webapp_url = Some(url.replace(match_str, &get_env(match_str.trim_matches(patterns))));
        }
    }


    // phaser
    for match_ in re.find_iter(&config.phaser.secret.clone()) {
        let match_str = match_.as_str();
        config.phaser.secret = config.phaser.secret.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // bitflow
    for match_ in re.find_iter(&config.bitflow.secret.clone()) {
        let match_str = match_.as_str();
        config.bitflow.secret = config.bitflow.secret.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // stripe
    for match_ in re.find_iter(&config.stripe.public_key.clone()) {
        let match_str = match_.as_str();
        config.stripe.public_key = config.stripe.public_key.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }
    for match_ in re.find_iter(&config.stripe.secret_key.clone()) {
        let match_str = match_.as_str();
        config.stripe.secret_key = config.stripe.secret_key.replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // blacklists
    config.blacklists.email_domains = config.blacklists.email_domains.into_iter().map(|mut item| {
        for match_ in re.find_iter(&item.clone()) {
            let match_str = match_.as_str();
            item = item.replace(match_str, &get_env(match_str.trim_matches(patterns)));
        }
        return item;
    }).collect();
    config.blacklists.passwords = config.blacklists.passwords.into_iter().map(|mut item| {
        for match_ in re.find_iter(&item.clone()) {
            let match_str = match_.as_str();
            item = item.replace(match_str, &get_env(match_str.trim_matches(patterns)));
        }
        return item;
    }).collect();

    return config;
}


pub fn get_env(var: &str) -> String {
    env::var(var).expect(&format!("Missing environment variable: {}", var)).to_string()
}

