# BobiBot Agent

A personal assistant agent built with Rust.

## Features

- Modular tool system for extensibility
- Async runtime powered by Tokio
- Configurable via environment variables
- Structured logging with tracing

## Requirements

- Rust 1.70 or later
- Cargo

## Installation

### Install Rust

If you don't have Rust installed, run:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run: https://rustup.rs/
```

### Build

```bash
cd bobibot-agent
cargo build --release
```

## Usage

```bash
# Run the agent
cargo run

# Or run the release build
./target/release/bobibot_agent
```

## Configuration

Set environment variables:

```bash
export LLM_API_ENDPOINT="https://api.openai.com/v1"
export LLM_API_KEY="your-api-key"
```

## Project Structure

```
bobibot-agent/
├── Cargo.toml          # Project dependencies
├── src/
│   ├── main.rs         # Entry point
│   ├── agent.rs        # Core agent logic
│   ├── config.rs       # Configuration management
│   └── tools.rs        # Tool implementations
└── README.md
```

## Extending

### Adding a New Tool

Implement the `Tool` trait in `src/tools.rs`:

```rust
pub struct MyTool;

impl Tool for MyTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    fn description(&self) -> &str {
        "Description of what the tool does"
    }

    fn execute(&self, input: &str) -> Result<String> {
        // Your implementation
        Ok("result".to_string())
    }
}
```

Then register it in the agent initialization.

## License

MIT
