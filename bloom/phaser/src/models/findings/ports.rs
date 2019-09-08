use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Port {
    pub id: u16,
    pub state: PortState,
    #[serde(skip)]
    pub http: bool,
    #[serde(skip)]
    pub https: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PortState {
    Open,
    Closed,
}
