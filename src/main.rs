use rmcp::transport::stdio;
use rmcp::ServiceExt;

use tracing_subscriber::EnvFilter;

use ulid_gen_mcp_rs::Ulid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server...");

    let service = Ulid::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("Error: {}", e);
        })
        .unwrap_or_else(|_| {
            std::process::exit(1);
        });

    service.waiting().await?;
    Ok(())
}
