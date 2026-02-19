//! BobiBot Agent - Personal Assistant Agent
//!
//! A Rust-based personal assistant agent that helps with daily tasks.

mod agent;
mod config;
mod tools;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting BobiBot Agent...");

    let config = config::Config::load()?;
    let mut agent = agent::Agent::new(config);

    agent.run().await?;

    tracing::info!("BobiBot Agent stopped.");
    Ok(())
}
