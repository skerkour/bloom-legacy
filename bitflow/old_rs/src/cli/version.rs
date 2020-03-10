use crate::info;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Debug, Serialize, Deserialize)]
struct Version<'a> {
    name: &'a str,
    version: &'a str,
    git_commit: &'a str,
    utc_build_time: &'a str,
    os: &'a str,
    architecture: &'a str,
    rust_version: &'a str,
}

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    println!("version");
    match matches.value_of("format") {
        Some("text") => print_text(),
        Some("json") => print_json(),
        _ => {
            println!("invalid format value");
            exit(1)
        }
    }
    Ok(())
}

// TODO: UTC build time, Git commit, Rust version
#[allow(clippy::print_literal)]
fn print_text() {
    println!(
        r#"Name           : {}
Version        : {}
Git commit     : {}
UTC build time : {}
OS             : {}
Architecture   : {}
Rust version   : {}
"#,
        info::NAME,
        info::VERSION,
        "",
        "",
        info::OS,
        info::ARCH,
        ""
    )
}

fn print_json() {
    let version = Version {
        name: info::NAME,
        version: info::VERSION,
        git_commit: "",
        utc_build_time: "",
        os: info::OS,
        architecture: info::ARCH,
        rust_version: "",
    };
    let json = serde_json::to_string(&version).expect("couldn't serialize config");
    println!("{}", json);
}
