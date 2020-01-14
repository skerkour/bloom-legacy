use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dmarc {
    pub domain: String,
    pub records: Vec<String>,
    pub resolves: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Spf {
    pub domain: String,
    pub records: Vec<String>,
    pub resolves: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Axfr {
    pub server: String,
    pub response: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Takeover {
    pub service: String,
}
