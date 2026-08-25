use crate::adapters::services::hydra::HydraService;
use crate::adapters::services::kratos::KratosService;
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::services::oidc::OidcService;
use crate::core::ports::utils::url_rewrite::UrlRewriter;
use crate::core::usecases::flow_handlers::login::LoginFlowHandler;
use crate::core::usecases::flow_handlers::recovery::RecoveryFlowHandler;
use crate::core::usecases::flow_handlers::registration::RegistrationFlowHandler;
use crate::core::usecases::flow_handlers::settings::SettingsFlowHandler;
use crate::core::usecases::flow_handlers::verification::VerificationFlowHandler;
use crate::infrastructure::utils::url_rewriter::AppInTheMiddle;
use crate::shared::config::Config;
use std::sync::Arc;
use crate::core::usecases::flow_handlers::consent::ConsentFlowHandler;
use crate::core::usecases::flow_handlers::error::ErrorFlowHandler;
use crate::core::usecases::flow_handlers::logout::LogoutFlowHandler;

pub struct AppState {
    pub config: Config,
    pub flow_handlers: FlowHandlers<KratosService, HydraService, AppInTheMiddle>,
}

pub struct FlowHandlers<I: IdentityService, O: OidcService, R: UrlRewriter> {
    pub login: Arc<LoginFlowHandler<I, O, R>>,
    pub registration: Arc<RegistrationFlowHandler<I, R>>,
    pub recovery: Arc<RecoveryFlowHandler<I, R>>,
    pub verification: Arc<VerificationFlowHandler<I, R>>,
    pub settings: Arc<SettingsFlowHandler<I, R>>,
    pub consent: Arc<ConsentFlowHandler<I, O>>,
    pub error: Arc<ErrorFlowHandler<I>>,
    pub logout: Arc<LogoutFlowHandler<I, O>>,
}
