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
adk = "0.2.1"
```

## Quick Start

Here's a simple example of creating an agent with custom tools(more examples in the `crates/adk/examples` directory):

```rust
use adk::agent::AgentBuilder;
use adk::openai::OpenAI;
use adk::prelude::*;
use adk::tool_fn;
use std::sync::Arc;

// Define a calculator tool using the tool_fn macro
#[tool_fn(
    name = "calculator",
    description = "A simple calculator that can perform basic arithmetic operations(add, subtract, multiply, divide)"
)]
fn calculator(_context: &mut RunContext, a: f64, b: f64, operation: String) -> String {
    // Perform the calculation based on the operation
    let result = match operation.as_str() {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => {
            if b == 0.0 {
                return "Error: Division by zero".to_string();
            }
            a / b
        }
        _ => return format!("Error: Invalid operation '{}'", operation),
    };

    // Return the result as a string
    result.to_string()
}

#[tokio::main]
async fn main() -> Result<(), AgentError> {
    // Initialize the OpenAI model
    let model = Arc::new(OpenAI::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        "gpt-4",
    ));

    // Create the calculator tool using the generated function from the macro
    let calculator = Arc::new(calculator_tool());

    // Create an agent using the builder pattern
    let agent = AgentBuilder::new("math_agent")
        .instructions("You are a helpful math assistant. Use the calculator tool to perform calculations when needed.")
        .model(model)
        .add_tool(calculator)
        .build()?;

    // Run the agent with a math problem
    let result = agent
        .run(
            "What is 15.7 * 9.2? Please use the calculator tool to compute this.",
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

- `crates/adk`: The main library crate with integrated macros
- `examples`: Example applications using the library. i.e. Run `cargo run -p examples --bin simple_agent` to see the calculator example.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 