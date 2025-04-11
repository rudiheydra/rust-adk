use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionFunctionCall, ChatCompletionFunctions, ChatCompletionRequestMessage,
        CreateChatCompletionRequest, Role,
    },
    Client,
};
use async_trait::async_trait;
use serde_json::Value;

use crate::error::{AgentError, AgentResult};
use crate::tool::Tool;
use crate::types::{Message, RunContext};

/// Trait for language models that can be used by agents
#[async_trait]
pub trait Model: Send + Sync {
    /// Generate a response based on the context and available tools
    async fn generate_response(
        &self,
        context: &mut RunContext,
        tools: &[&dyn Tool],
    ) -> AgentResult<String>;
}

/// OpenAI model implementation
pub struct OpenAI {
    client: Client<OpenAIConfig>,
    model: String,
    temperature: f32,
    max_tokens: Option<u16>,
}

impl OpenAI {
    pub fn new(
        api_key: impl Into<String>,
        model: impl Into<String>,
        temperature: f32,
        max_tokens: Option<u16>,
    ) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        Self {
            client,
            model: model.into(),
            temperature,
            max_tokens,
        }
    }

    fn create_messages(&self, context: &RunContext) -> Vec<ChatCompletionRequestMessage> {
        context
            .messages
            .iter()
            .map(|msg| {
                let role = match msg.role.as_str() {
                    "system" => Role::System,
                    "user" => Role::User,
                    "assistant" => Role::Assistant,
                    "tool" => Role::Function,
                    _ => Role::User,
                };
                ChatCompletionRequestMessage {
                    role,
                    content: Some(msg.content.clone()),
                    name: msg.tool_name.clone(),
                    function_call: None,
                }
            })
            .collect()
    }

    fn create_functions(&self, tools: &[&dyn Tool]) -> Vec<ChatCompletionFunctions> {
        tools
            .iter()
            .map(|tool| ChatCompletionFunctions {
                name: tool.name().to_string(),
                description: Some(tool.description().to_string()),
                parameters: tool.parameters_schema(),
            })
            .collect()
    }
}

#[async_trait]
impl Model for OpenAI {
    async fn generate_response(
        &self,
        context: &mut RunContext,
        tools: &[&dyn Tool],
    ) -> AgentResult<String> {
        let messages = self.create_messages(context);
        let functions = self.create_functions(tools);

        let request = CreateChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: Some(self.temperature),
            max_tokens: self.max_tokens,
            functions: Some(functions),
            function_call: Some(ChatCompletionFunctionCall::Auto),
            ..Default::default()
        };

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(AgentError::OpenAIError)?;

        let message = response
            .choices
            .first()
            .ok_or_else(|| AgentError::ModelError("No response from model".into()))?
            .message
            .clone();

        if let Some(function_call) = message.function_call {
            // Handle function call
            let tool = tools
                .iter()
                .find(|tool| tool.name() == function_call.name)
                .ok_or_else(|| {
                    AgentError::ToolError(format!("Tool not found: {}", function_call.name))
                })?;

            let result = tool.execute(context, &function_call.arguments).await?;

            context.add_tool_message(result.tool_name, result.output);

            // Recursively generate another response with the tool result
            self.generate_response(context, tools).await
        } else {
            Ok(message.content.unwrap_or_default())
        }
    }
}
