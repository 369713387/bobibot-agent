//! Agent module - Core agent implementation

use crate::config::Config;
use crate::tools::ToolRegistry;
use anyhow::Result;

pub struct Agent {
    config: Config,
    tools: ToolRegistry,
}

impl Agent {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tools: ToolRegistry::new(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Agent initialized with name: {}", self.config.name);

        // TODO: Implement main agent loop
        // 1. Receive user input
        // 2. Process with LLM
        // 3. Execute tools if needed
        // 4. Return response

        println!("BobiBot Agent is ready! (Press Ctrl+C to exit)");

        // Placeholder: keep the agent running
        tokio::signal::ctrl_c().await?;
        println!("\nShutting down...");

        Ok(())
    }
}
