//! Tools module - Agent tools/functions

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A tool that the agent can use
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String>;
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        tracing::info!("Registered tool: {}", tool.name());
        self.tools.push(tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }

    pub fn list(&self) -> Vec<&str> {
        self.tools.iter().map(|t| t.name()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Example tool implementations

/// Echo tool for testing
pub struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echoes back the input text"
    }

    fn execute(&self, input: &str) -> Result<String> {
        Ok(input.to_string())
    }
}

/// Current time tool
pub struct TimeTool;

impl Tool for TimeTool {
    fn name(&self) -> &str {
        "time"
    }

    fn description(&self) -> &str {
        "Returns the current date and time"
    }

    fn execute(&self, _input: &str) -> Result<String> {
        let now = chrono_lite();
        Ok(now)
    }
}

fn chrono_lite() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let secs = duration.as_secs();
    // Simple time formatting
    format!("Unix timestamp: {} seconds since epoch", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_tool() {
        let tool = EchoTool;
        assert_eq!(tool.execute("hello").unwrap(), "hello");
    }

    #[test]
    fn test_tool_registry() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(EchoTool));
        assert_eq!(registry.list(), vec!["echo"]);
    }
}
