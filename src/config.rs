use std::env;
use serde::{Serialize, Deserialize};
use dotenv;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    port: String,
    database_url: String,
    rust_env: String,
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
    www_host: String,
    aws_access_key: String,
    aws_secret_access_key: String,
    aws_region: String,
    aws_s3_bucket: String,
    github_token: String,
    sentry_url: String,
    phaser_secret: String,
    bitflow_secret: String,
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

    return Config{
        port: get_env("PORT"),
        database_url: get_env("DATABASE_URL"),
        rust_env: get_env("RUST_ENV"),
        smtp_host: get_env("SMTP_HOST"),
        smtp_username: get_env("SMTP_USERNAME"),
        smtp_password: get_env("SMTP_PASSWORD"),
        www_host: get_env("WWW_HOST"),
        aws_access_key: get_env("AWS_ACCESS_KEY"),
        aws_secret_access_key: get_env("AWS_SECRET_ACCESS_KEY"),
        aws_region: get_env("AWS_REGION"),
        aws_s3_bucket: get_env("AWS_S3_BUCKET"),
        github_token: get_env("GITHUB_TOKEN"),
        sentry_url: get_env("SENTRY_URL"),
        phaser_secret: get_env("PHASER_SECRET"),
        bitflow_secret: get_env("BITFLOW_SECRET"),
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

    pub fn www_host(&self) -> String {
        return self.www_host.clone();
    }

    pub fn aws_access_key(&self) -> String {
        return self.aws_access_key.clone();
    }

    pub fn aws_secret_access_key(&self) -> String {
        return self.aws_secret_access_key.clone();
    }

    pub fn aws_region(&self) -> String {
        return self.aws_region.clone();
    }

    pub fn aws_s3_bucket(&self) -> String {
        return self.aws_s3_bucket.clone();
    }

    pub fn github_token(&self) -> String {
        return self.github_token.clone();
    }

    pub fn sentry_url(&self) -> String {
        return self.sentry_url.clone();
    }

    pub fn phaser_secret(&self) -> String {
        return self.phaser_secret.clone();
    }

    pub fn bitflow_secret(&self) -> String {
        return self.bitflow_secret.clone();
    }
}


pub fn get_env(var: &str) -> String {
    env::var(var).expect(&format!("Missing environment variable: {}", var)).to_string()
}
