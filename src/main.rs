use std::sync::{Arc, Mutex};

use rmcp::transport::stdio;
use rmcp::{
    ServerHandler, ServiceExt,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool,
};

use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct Ulid {
    ulid: Arc<Mutex<ulid::Ulid>>,
}

#[tool(tool_box)]
impl Ulid {
    fn new() -> Self {
        Self {
            ulid: Arc::new(Mutex::new(ulid::Ulid::new())),
        }
    }

    #[tool(description = "generate a ULID")]
    fn generate(&self) -> anyhow::Result<CallToolResult, rmcp::Error> {
        let mut ulid = self.ulid.lock().unwrap();
        *ulid = ulid::Ulid::new();
        tracing::info!("Generated new ULID: {}", ulid);
        Ok(CallToolResult::success(vec![Content::text(
            ulid.to_string(),
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Ulid {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This is a server for generating ULID".into()),
        }
    }
}

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
