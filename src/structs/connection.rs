use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub key: String,
    pub protocol: String,
    pub pbk: Option<String>,
    pub sni: Option<String>,
}
