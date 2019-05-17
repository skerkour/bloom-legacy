mod ports;
mod file;
mod url;


use serde::{Serialize, Deserialize};

pub use ports::{Port, PortState};
pub use file::File;
pub use self::url::Url;
pub mod domain;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Finding {
    pub module_version: String,
    pub result: ModuleResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ModuleResult {
    None,
    Err(String),
    Ports(Vec<Port>),
    File(File),
    Domain(String),
    Domains(Vec<String>),
    Axfr(Vec<domain::Axfr>),
    Dmarc(domain::Dmarc),
    Spf(domain::Spf),
    Takeover(domain::Takeover),
    Url(Url),
}
