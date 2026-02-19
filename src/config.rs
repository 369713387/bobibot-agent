//! Configuration module

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Agent name
    pub name: String,
    /// LLM API endpoint
    pub api_endpoint: Option<String>,
    /// API key (loaded from environment)
    pub api_key: Option<String>,
    /// Model to use
    pub model: String,
    /// Maximum tokens for responses
    pub max_tokens: usize,
    /// Temperature for generation
    pub temperature: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "BobiBot".to_string(),
            api_endpoint: std::env::var("LLM_API_ENDPOINT").ok(),
            api_key: std::env::var("LLM_API_KEY").ok(),
            model: "gpt-4".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // TODO: Load from config file (config.toml or config.yaml)
        // For now, use defaults with environment variables
        Ok(Self::default())
    }
}
