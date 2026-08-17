use ory_selfservice_rust::infrastructure::bootstrap;
use ory_selfservice_rust::shared::error::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    bootstrap::start().await
}
