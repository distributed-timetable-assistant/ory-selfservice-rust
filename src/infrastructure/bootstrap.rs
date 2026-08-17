use crate::http::middleware::trace_request_context;
use crate::http::routes::create_router;
use crate::hydra::client::HydraClient;
use crate::infrastructure::cli::Cli;
use crate::infrastructure::{config_loader, telemetry};
use crate::kratos::client::KratosClient;
use crate::shared::config::Config;
use crate::shared::error::AppResult;
use crate::shared::state::AppState;
use axum::middleware;
use clap::Parser;
use tracing::info;

pub async fn start() -> AppResult<()> {
    telemetry::init();
    info!("Starting Ory Shield UI service...");

    let cli = Cli::parse();
    let conf_path = cli.config;
    let config: Config = config_loader::load(&conf_path);

    // Base reqwest client that doesn't follow redirects automatically
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // Initialize Kratos client
    let kratos_client = KratosClient::new(config.kratos_public_url.clone(), client.clone());

    // Initialize Hydra client
    let hydra_client = HydraClient::new(config.hydra_admin_url.clone());

    // Setup state
    let state = AppState {
        kratos: kratos_client,
        hydra: hydra_client,
        config: config.clone(),
    };

    // Create router with telemetry middleware
    let app = create_router(state).layer(middleware::from_fn(trace_request_context));

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
