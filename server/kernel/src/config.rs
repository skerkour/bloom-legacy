use std::env;
use serde::{Serialize, Deserialize};
use dotenv;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    port: String,
    database_url: String,
    rust_env: String,
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
    host: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
    s3_bucket: String,
    s3_base_url: String,
    github_token: String,
    sentry_url: Option<String>,
    phaser_secret: String,
    bitflow_secret: String,
    disposable_email_domains: Vec<String>,
    // stripe_secret_key: String,
}

pub fn init() -> Config {
    dotenv::dotenv().ok();

    // set default env vars
    if env::var("PORT").is_err() {
        env::set_var("PORT", "8000")
    }

    if env::var("RUST_ENV").is_err() {
        env::set_var("RUST_ENV", "development")
    }

    let disposable_emails_file = File::open("assets/disposable_email_domains.txt").expect("Error opening disposable email file");
    let disposable_email_domains: Vec<String> = BufReader::new(disposable_emails_file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    return Config{
        port: get_env("PORT"),
        database_url: get_env("DATABASE_URL"),
        rust_env: get_env("RUST_ENV"),
        smtp_host: get_env("SMTP_HOST"),
        smtp_username: get_env("SMTP_USERNAME"),
        smtp_password: get_env("SMTP_PASSWORD"),
        host: get_env("HOST"),
        aws_access_key_id: get_env("AWS_ACCESS_KEY_ID"),
        aws_secret_access_key: get_env("AWS_SECRET_ACCESS_KEY"),
        aws_region: get_env("AWS_REGION"),
        s3_bucket: get_env("S3_BUCKET"),
        s3_base_url: get_env("S3_BASE_URL"),
        github_token: get_env("GITHUB_TOKEN"),
        sentry_url: env::var("SENTRY_URL").ok(),
        phaser_secret: get_env("PHASER_SECRET"),
        bitflow_secret: get_env("BITFLOW_SECRET"),
        disposable_email_domains,
        // stripe_secret_key: get_env("STRIPE_SECRET_KEY"),
    };
}


impl Config {
    pub fn port(&self) -> String {
        return self.port.clone();
    }

    pub fn database_url(&self) -> String {
        return self.database_url.clone();
    }

    pub fn rust_env(&self) -> String {
        return self.rust_env.clone();
    }

    pub fn smtp_host(&self) -> String {
        return self.smtp_host.clone();
    }

    pub fn smtp_username(&self) -> String {
        return self.smtp_username.clone();
    }

    pub fn smtp_password(&self) -> String {
        return self.smtp_password.clone();
    }

    pub fn host(&self) -> String {
        return self.host.clone();
    }

    pub fn aws_access_key_id(&self) -> String {
        return self.aws_access_key_id.clone();
    }

    pub fn aws_secret_access_key(&self) -> String {
        return self.aws_secret_access_key.clone();
    }

    pub fn aws_region(&self) -> String {
        return self.aws_region.clone();
    }

    pub fn s3_bucket(&self) -> String {
        return self.s3_bucket.clone();
    }

    pub fn s3_base_url(&self) -> String {
        return self.s3_base_url.clone();
    }

    pub fn github_token(&self) -> String {
        return self.github_token.clone();
    }

    pub fn sentry_url(&self) -> Option<String> {
        return self.sentry_url.clone();
    }

    pub fn phaser_secret(&self) -> String {
        return self.phaser_secret.clone();
    }

    pub fn bitflow_secret(&self) -> String {
        return self.bitflow_secret.clone();
    }

    pub fn disposable_email_domains(&self) -> Vec<String> {
        return self.disposable_email_domains.clone();
    }

    // pub fn stripe_secret_key(&self) -> String {
    //     return self.stripe_secret_key.clone();
    // }
}


pub fn get_env(var: &str) -> String {
    env::var(var).expect(&format!("Missing environment variable: {}", var)).to_string()
}
