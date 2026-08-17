use crate::hydra::client::HydraClient;
use crate::kratos::client::KratosClient;
use crate::shared::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub kratos: KratosClient,
    pub hydra: HydraClient,
    pub config: Config,
}
