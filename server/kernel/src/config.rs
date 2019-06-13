use std::env;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::fs;
use regex::Regex;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    rust_env: String,
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
    // stripe_secret_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    rust_env: String,
    host: String,
    port: u16,
    smtp: SmtpConfig,
    database: DatabaseConfig,
    aws: AwsConfig,
    s3: S3Config,
    sentry: SentryConfig,
    phaser: PhaserConfig,
    bitflow: BitflowConfig,
    disposable_email_domains: HashSet<String>,
    basic_passwords: HashSet<String>,
    // stripe_secret_key: String,
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
            disposable_email_domains: blacklisted_email_domains,
            basic_passwords: blacklisted_passwords,
        };
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
struct AwsConfig {
    access_key_id: String,
    secret_access_key: String,
    region: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct S3Config {
    bucket: String,
    base_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PhaserConfig {
    secret: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BitflowConfig {
    secret: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SmtpConfig {
    host: String,
    username: String,
    password: String,
    port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SentryConfig {
    url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    url: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
struct BlacklistsConfig {
    email_domains: Vec<String>,
    passwords: Vec<String>,
}

pub fn init() -> Config {
    let config_file_contents = fs::read_to_string("bloom.sane")
        .expect("Error reading bloom.sane");

    let decoded: ConfigFile = sane::from_str(&config_file_contents)
        .expect("Error parsing config file");

    let decoded = replace_env(decoded);

    return decoded.into();
}


impl Config {
    pub fn port(&self) -> String {
        return self.port.to_string();
    }

    pub fn database_url(&self) -> String {
        return self.database.url.clone();
    }

    pub fn rust_env(&self) -> String {
        return self.rust_env.clone();
    }

    pub fn smtp_host(&self) -> String {
        return self.smtp.host.clone();
    }

    pub fn smtp_username(&self) -> String {
        return self.smtp.username.clone();
    }

    pub fn smtp_password(&self) -> String {
        return self.smtp.password.clone();
    }

    pub fn smtp_port(&self) -> u16 {
        return self.smtp.port;
    }

    pub fn host(&self) -> String {
        return self.host.clone();
    }

    pub fn aws_access_key_id(&self) -> String {
        return self.aws.access_key_id.clone();
    }

    pub fn aws_secret_access_key(&self) -> String {
        return self.aws.secret_access_key.clone();
    }

    pub fn aws_region(&self) -> String {
        return self.aws.region.clone();
    }

    pub fn s3_bucket(&self) -> String {
        return self.s3.bucket.clone();
    }

    pub fn s3_base_url(&self) -> String {
        return self.s3.base_url.clone();
    }

    pub fn sentry_url(&self) -> Option<String> {
        return self.sentry.url.clone();
    }

    pub fn phaser_secret(&self) -> String {
        return self.phaser.secret.clone();
    }

    pub fn bitflow_secret(&self) -> String {
        return self.bitflow.secret.clone();
    }

    pub fn disposable_email_domains(&self) -> HashSet<String> {
        return self.disposable_email_domains.clone();
    }

    pub fn basic_passwords(&self) -> HashSet<String> {
        return self.basic_passwords.clone();
    }

    // pub fn stripe_secret_key(&self) -> String {
    //     return self.stripe_secret_key.clone();
    // }
}


fn replace_env(config: ConfigFile) -> ConfigFile {
    let re = Regex::new(r"\$\{[A-Z_0-9]*\}").expect("error compiling env regex");

    for match_ in re.find_iter(&config.host) {
        println!("{}", match_.as_str());
    }

    return config;
}


pub fn get_env(var: &str) -> String {
    env::var(var).expect(&format!("Missing environment variable: {}", var)).to_string()
}

