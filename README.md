# Rust Agent Kit

A Rust implementation of the OpenAI agents kit, providing a framework for building AI agents that can use tools and interact with language models.

## Features

- Easy-to-use agent framework
- Support for OpenAI models
- Tool system for extending agent capabilities
- Context management for maintaining state
- Async/await support
- Type-safe API

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust-agent-kit = "0.1.0"
```

## Quick Start

Here's a simple example of creating an agent with a custom tool:

```rust
use rust_agent_kit::prelude::*;
use std::sync::Arc;

// Define a simple calculator tool
struct CalculatorTool;

#[async_trait::async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "A simple calculator that can perform basic arithmetic operations"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"]
                },
                "a": {
                    "type": "number"
                },
                "b": {
                    "type": "number"
                }
            },
            "required": ["operation", "a", "b"]
        })
    }

    async fn execute(&self, _context: &mut RunContext, params: &str) -> AgentResult<ToolResult> {
        // Implementation details...
    }
}

#[tokio::main]
async fn main() -> AgentResult<()> {
    // Initialize the OpenAI model
    let model = Arc::new(OpenAI::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        "gpt-4",
        0.7,
        Some(1000),
    ));

    // Create the calculator tool
    let calculator = Arc::new(CalculatorTool);

    // Create an agent using the builder pattern
    let agent = AgentBuilder::new("math_agent")
        .instructions("You are a helpful math assistant. Use the calculator tool to perform calculations when needed.")
        .model(model)
        .add_tool(calculator)
        .build()?;

    // Run the agent with a math problem
    let result = agent
        .run(
            "What is 123 + 456? Please use the calculator tool to compute this.",
            Context::new(),
        )
        .await?;

    println!("Agent response: {}", result);

    Ok(())
}
```

## Core Components

### Agent

The `Agent` struct is the main entry point for interacting with the language model. It manages the conversation context and tool execution.

### Tools

Tools are the way agents interact with the outside world. They can be used to perform actions, retrieve information, or modify state.

### Models

Models handle the interaction with language models. Currently, OpenAI models are supported, but the framework is designed to be extensible.

### Context

The `Context` and `RunContext` types manage the state of the conversation and provide a way to share data between tools and the agent.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 