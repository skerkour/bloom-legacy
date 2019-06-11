use std::env;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::fs;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    port: u16,
    database: DatabaseConfig,
    rust_env: String,
    smtp: SmtpConfig,
    host: String,
    aws: AwsConfig,
    s3: S3Config,
    sentry: SentryConfig,
    phaser: PhaserConfig,
    bitflow: BitflowConfig,
    #[serde(skip)]
    disposable_email_domains: HashSet<String>,
    // stripe_secret_key: String,
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

pub fn init() -> Config {
    let config_file_contents = fs::read_to_string("bloom.sane")
        .expect("Error reading bloom.sane");

    let mut decoded: Config = sane::from_str(&config_file_contents)
        .expect("Error parsing config file");


    let disposable_emails_file = File::open("assets/disposable_email_domains.txt").expect("Error opening disposable email file");
    let disposable_email_domains: Vec<String> = BufReader::new(disposable_emails_file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let disposable_email_domains = disposable_email_domains.iter().fold(HashSet::new(), |mut acc, x| {
        acc.insert(x.to_string());
        return acc;
    });

    decoded.disposable_email_domains = disposable_email_domains;

    return decoded;
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

    // pub fn stripe_secret_key(&self) -> String {
    //     return self.stripe_secret_key.clone();
    // }
}


pub fn get_env(var: &str) -> String {
    env::var(var).expect(&format!("Missing environment variable: {}", var)).to_string()
}
