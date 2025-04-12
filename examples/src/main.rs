use rust_adk::error::AgentResult;
use rust_adk::prelude::*;
use rust_adk::tool::{Tool, ToolResult, tool_fn};
use rust_adk::types::RunContext;

// Define a calculator tool using our procedural macro
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

// Use the traditional method of creating a tool for comparison
fn create_manual_calculator_tool() -> rust_adk::tool::FunctionTool {
    use rust_adk::function_tool;

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
async fn main() {
    // Create a calculator tool using our procedural macro
    let calc_tool = calculator_tool();

    // Create a manual calculator tool
    let manual_calc_tool = create_manual_calculator_tool();

    // Create tool references that implement the Tool trait
    let calc_tool_ref: &dyn Tool = &calc_tool;
    let manual_calc_tool_ref: &dyn Tool = &manual_calc_tool;

    // Print the parameter schemas to compare
    println!("Auto-generated schema from tool_fn:");
    println!(
        "{}",
        serde_json::to_string_pretty(&calc_tool_ref.parameters_schema()).unwrap()
    );

    println!("\nManually defined schema:");
    println!(
        "{}",
        serde_json::to_string_pretty(&manual_calc_tool_ref.parameters_schema()).unwrap()
    );

    // Test executing the tool
    let mut context = RunContext::new(Context::new());

    // Prepare test parameters
    let params = r#"{"a": 10, "b": 5, "operation": "add"}"#;

    // Execute the tool
    match calc_tool_ref.execute(&mut context, params).await {
        Ok(result) => println!("\nTool execution result: {}", result.output),
        Err(e) => println!("\nError executing tool: {}", e),
    }
}
