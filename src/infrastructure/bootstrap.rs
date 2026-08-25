use crate::adapters::http::routes::router;
use crate::adapters::services::hydra::HydraService;
use crate::adapters::services::kratos::KratosService;
use crate::core::ports::services::identity::IdentityService;
use crate::core::ports::services::oidc::OidcService;
use crate::core::ports::utils::url_rewrite::UrlRewriter;
use crate::core::usecases::flow_handlers::consent::ConsentFlowHandler;
use crate::core::usecases::flow_handlers::error::ErrorFlowHandler;
use crate::core::usecases::flow_handlers::login::LoginFlowHandler;
use crate::core::usecases::flow_handlers::logout::LogoutFlowHandler;
use crate::core::usecases::flow_handlers::recovery::RecoveryFlowHandler;
use crate::core::usecases::flow_handlers::registration::RegistrationFlowHandler;
use crate::core::usecases::flow_handlers::settings::SettingsFlowHandler;
use crate::core::usecases::flow_handlers::verification::VerificationFlowHandler;
use crate::infrastructure::cli::Cli;
use crate::infrastructure::utils::url_rewriter::AppInTheMiddle;
use crate::infrastructure::{config_loader, telemetry};
use crate::shared::config::Config;
use crate::shared::error::{AppError, AppResult};
use crate::shared::state::{AppState, FlowHandlers};
use clap::Parser;
use reqwest_middleware::ClientBuilder;
use std::sync::Arc;
use reqwest_tracing::TracingMiddleware;
use tower_http::trace::TraceLayer;
use tracing::info;
use url::Url;

pub async fn start() -> AppResult<()> {
    telemetry::init();
    info!("Starting Ory Shield UI service...");

    let cli = Cli::parse();
    let conf_path = cli.config;
    let config: Config = config_loader::load(&conf_path);

    // Base reqwest client that doesn't follow redirects automatically
    let raw_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let client = ClientBuilder::new(raw_client)
        .with(TracingMiddleware::default())
        .build();
    // Initialize Kratos client
    let kratos_service = Arc::new(KratosService::new(
        config.kratos_public_url.clone(),
        client.clone(),
    ));

    // Initialize Hydra client
    let hydra_service = Arc::new(HydraService::new(
        config.hydra_admin_url.clone(),
        client,
    ));

    // Initialize Url Rewriter Util
    let app_url: Url = config
        .public_base_url
        .parse()
        .map_err(|e| AppError::Url(e))?;
    let kratos_url: Url = config
        .kratos_public_url
        .parse()
        .map_err(|e| AppError::Url(e))?;
    let app_in_the_middle = Arc::new(AppInTheMiddle::new(app_url, kratos_url));

    // Initialize Flows
    let flow_handlers = get_flow_handlers(kratos_service, hydra_service, app_in_the_middle);

    // Setup state
    let state = AppState {
        config: config.clone(),
        flow_handlers,
    };

    // Create router with telemetry middleware
    let app = router()
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http());

    // Bind listener
    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;
    info!("Listening -> SUCCESS: listen_addr=({})", config.listen_addr);

    axum::serve(listener, app).await?;
    info!(
        "Axum Serve -> SUCCESS: listen_addr=({})",
        config.listen_addr
    );

    Ok(())
}

fn get_flow_handlers<I: IdentityService, O: OidcService, R: UrlRewriter>(
    identity_service: Arc<I>,
    oidc_service: Arc<O>,
    url_rewriter: Arc<R>,
) -> FlowHandlers<I, O, R> {
    FlowHandlers {
        login: Arc::new(LoginFlowHandler::new(
            identity_service.clone(),
            oidc_service.clone(),
            url_rewriter.clone(),
        )),
        registration: Arc::new(RegistrationFlowHandler::new(
            identity_service.clone(),
            url_rewriter.clone(),
        )),
        recovery: Arc::new(RecoveryFlowHandler::new(
            identity_service.clone(),
            url_rewriter.clone(),
        )),
        verification: Arc::new(VerificationFlowHandler::new(
            identity_service.clone(),
            url_rewriter.clone(),
        )),
        settings: Arc::new(SettingsFlowHandler::new(
            identity_service.clone(),
            url_rewriter,
        )),
        consent: Arc::new(ConsentFlowHandler::new(
            identity_service.clone(),
            oidc_service.clone(),
        )),
        error: Arc::new(ErrorFlowHandler::new(identity_service.clone())),
        logout: Arc::new(LogoutFlowHandler::new(identity_service, oidc_service)),
    }
}
