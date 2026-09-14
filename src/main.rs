use mimalloc::MiMalloc;
use ory_selfservice_rust::infrastructure::bootstrap;
use ory_selfservice_rust::shared::error::AppResult;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> AppResult<()> {
    bootstrap::start().await
}
