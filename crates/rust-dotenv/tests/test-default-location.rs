extern crate dotenv;
extern crate tempdir;

mod common;

use std::env;
use dotenv::*;

use crate::common::*;

#[test]
fn test_default_location() {
    let dir = make_test_dotenv().unwrap();

    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
