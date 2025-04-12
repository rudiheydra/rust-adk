# Rust ADK(Agent Development Kit)

A Rust implementation of the ADK(Agent Development Kit), providing a framework for building AI agents that can use tools and interact with language models.

## Features

- Easy-to-use agent framework
- Tool system for extending agent capabilities with multiple implementation approaches
- Context management for maintaining state
- Async/await support
- Type-safe API

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
adk = "0.1.0"
```

## Quick Start

Here's a simple example of creating an agent with custom tools:

```rust
use adk::error::AgentResult;
use adk::prelude::*;
use adk::tool::{tool_fn, Tool, ToolResult};
use adk::types::RunContext;
use std::sync::Arc;

// Define a calculator tool using the procedural macro
#[tool_fn(
    name = "calculator",
    description = "A simple calculator that performs basic arithmetic"
)]
fn calculator(_context: &mut RunContext, a: i32, b: i32, operation: String) -> String {
    match operation.as_str() {
        "add" => format!("{} + {} = {}", a, b, a + b),
        "subtract" => format!("{} - {} = {}", a, b, a - b),
        "multiply" => format!("{} * {} = {}", a, b, a * b),
        "divide" => {
            if b == 0 {
                "Error: Division by zero".to_string()
            } else {
                format!("{} / {} = {}", a, b, a / b)
            }
        }
        _ => format!("Unknown operation: {}", operation),
    }
}

// Alternatively, define a tool using the function_tool macro
fn create_manual_calculator_tool() -> adk::tool::FunctionTool {
    use adk::function_tool;

    function_tool!(
        "manual_calculator",
        "A manually defined calculator tool",
        |_context: &mut RunContext, params: &str| -> AgentResult<ToolResult> {
            let params: serde_json::Value = serde_json::from_str(params)?;

            let a = params["a"].as_i64().unwrap_or(0) as i32;
            let b = params["b"].as_i64().unwrap_or(0) as i32;
            let operation = params["operation"].as_str().unwrap_or("add");

            let result = match operation {
                "add" => format!("{} + {} = {}", a, b, a + b),
                "subtract" => format!("{} - {} = {}", a, b, a - b),
                "multiply" => format!("{} * {} = {}", a, b, a * b),
                "divide" => {
                    if b == 0 {
                        "Error: Division by zero".to_string()
                    } else {
                        format!("{} / {} = {}", a, b, a / b)
                    }
                }
                _ => format!("Unknown operation: {}", operation),
            };

            Ok(ToolResult {
                tool_name: "manual_calculator".to_string(),
                output: result,
            })
        }
    )
}

#[tokio::main]
async fn main() -> AgentResult<()> {
    // Initialize your model - Note: OpenAI reference implementation is still in progress
    // For now, you would need to implement the Model trait for your specific LLM provider
    let model = Arc::new(YourModelImplementation::new(
        "your-api-key",
        "model-name",
        0.7,  // temperature
    ));

    // Create the calculator tools
    let calc_tool = Arc::new(calculator_tool());
    let manual_calc_tool = Arc::new(create_manual_calculator_tool());

    // Create an agent using the builder pattern
    let agent = AgentBuilder::new("math_agent")
        .instructions("You are a helpful math assistant. Use the calculator tools to perform calculations when needed.")
        .model(model)
        .add_tool(calc_tool)
        .add_tool(manual_calc_tool)
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

```rust
// Create an agent with the builder pattern
let agent = AgentBuilder::new("my_agent")
    .instructions("You are a helpful assistant...")
    .model(model)
    .add_tool(tool)
    .build()?;
```

### Tools

Tools are the way agents interact with the outside world. There are multiple ways to define tools:

1. Using the procedural macro:
```rust
#[tool_fn(
    name = "tool_name",
    description = "Tool description"
)]
fn my_tool(context: &mut RunContext, param1: Type1, param2: Type2) -> String {
    // Implementation
}
```

2. Using the function_tool macro:
```rust
function_tool!(
    "tool_name",
    "Tool description",
    |context, params| -> AgentResult<ToolResult> {
        // Implementation
    }
)
```

3. Implementing the Tool trait directly:
```rust
struct MyTool;

#[async_trait::async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str { "tool_name" }
    fn description(&self) -> &str { "Tool description" }
    fn parameters_schema(&self) -> serde_json::Value { /* ... */ }
    async fn execute(&self, context: &mut RunContext, params: &str) -> AgentResult<ToolResult> {
        // Implementation
    }
}
```

### Models

Models handle the interaction with language models. The framework defines a `Model` trait that must be implemented for your specific LLM provider.

```rust
#[async_trait::async_trait]
impl Model for YourModel {
    async fn generate_response(
        &self, 
        context: &mut RunContext, 
        tools: &[&dyn Tool]
    ) -> AgentResult<String> {
        // Implementation
    }
}
```

> Note: The OpenAI model reference implementation is a work in progress.

### Context

The `Context` and `RunContext` types manage the state of the conversation and provide a way to share data between tools and the agent.

## Project Structure

The project is organized into the following components:

- `crates/adk`: The main library crate
- `crates/adk-macros`: Procedural macros for tool definitions
- `examples`: Example applications using the library

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 