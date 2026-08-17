use clap::Parser;

#[derive(Parser)]
#[command(name = "ory-selfservice-rust")]
#[command(about = "Ory Selfservice", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub config: Option<String>,
}
