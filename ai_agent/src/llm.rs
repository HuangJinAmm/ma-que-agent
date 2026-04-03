use crate::tools::ToolDefinition;
use genai::chat::{ChatMessage, ChatRequest, Tool, ToolCall};
use genai::Client;

pub struct LlmClient {
    client: Client,
    model: String,
}

#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
}

impl LlmClient {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            client: Client::default(),
            model: model.into(),
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<Vec<ToolDefinition>>,
    ) -> Result<LlmResponse, genai::Error> {
        let mut request = ChatRequest::new(messages);

        if let Some(tool_defs) = tools {
            let genai_tools = tool_defs
                .into_iter()
                .map(|td| {
                    let mut tool = Tool::new(&td.name);
                    if let Some(desc) = td.description {
                        tool = tool.with_description(&desc);
                    }
                    if let Ok(schema_value) = serde_json::to_value(&td.input_schema) {
                        tool = tool.with_schema(schema_value);
                    }
                    tool
                })
                .collect::<Vec<_>>();
            request = request.with_tools(genai_tools);
        }

        let response = self.client.exec_chat(&self.model, request, None).await?;

        let content = response
            .first_text()
            .unwrap_or_default()
            .to_string();

        let tool_calls = response
            .content
            .tool_calls()
            .into_iter()
            .cloned()
            .collect();

        Ok(LlmResponse {
            content,
            tool_calls,
        })
    }
}
