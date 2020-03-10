use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rust_env: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub aws_s3_bucket: String,
    pub sentry_url: String,
    pub bloom_api_url: String,
    pub bloom_api_secret: String,
    pub concurrency: u32,
}

fn get_env(var: &str) -> String {
    match env::var(var) {
        Ok(v) => v,
        Err(_err) => panic!("missing ENV var: {}", var),
    }
}

impl Config {
    pub fn new() -> Config {
        let config_file_contents =
            fs::read_to_string("bitflow.sane").expect("Error reading bitflow.sane");

        let decoded: Config =
            sane::from_str(&config_file_contents).expect("Error parsing config file");

        env::set_var("AWS_ACCESS_KEY_ID", decoded.aws_access_key_id.clone());
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            decoded.aws_secret_access_key.clone(),
        );
        env::set_var("AWS_REGION", decoded.aws_region.clone());

        return replace_env(decoded);
    }
}

fn replace_env(mut config: Config) -> Config {
    let re = Regex::new(r"\$\{[A-Z_0-9]*\}").expect("error compiling env regex");
    let patterns: &[_] = &['$', '{', '}'];

    // rust_env
    for match_ in re.find_iter(&config.rust_env.clone()) {
        let match_str = match_.as_str();
        config.rust_env = config
            .rust_env
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // aws_access_key_id
    for match_ in re.find_iter(&config.aws_access_key_id.clone()) {
        let match_str = match_.as_str();
        config.aws_access_key_id = config
            .aws_access_key_id
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // aws_secret_access_key
    for match_ in re.find_iter(&config.aws_secret_access_key.clone()) {
        let match_str = match_.as_str();
        config.aws_secret_access_key = config
            .aws_secret_access_key
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // aws_s3_bucket
    for match_ in re.find_iter(&config.aws_s3_bucket.clone()) {
        let match_str = match_.as_str();
        config.aws_s3_bucket = config
            .aws_s3_bucket
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // sentry_url
    for match_ in re.find_iter(&config.sentry_url.clone()) {
        let match_str = match_.as_str();
        config.sentry_url = config
            .sentry_url
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // bloom_api_url
    for match_ in re.find_iter(&config.bloom_api_url.clone()) {
        let match_str = match_.as_str();
        config.bloom_api_url = config
            .bloom_api_url
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    // bloom_api_secret
    for match_ in re.find_iter(&config.bloom_api_secret.clone()) {
        let match_str = match_.as_str();
        config.bloom_api_secret = config
            .bloom_api_secret
            .replace(match_str, &get_env(match_str.trim_matches(patterns)));
    }

    return config;
}
