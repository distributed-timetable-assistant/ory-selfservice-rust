use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub public_base_url: String,
    pub kratos_public_url: String,
    pub hydra_admin_url: String,
}
