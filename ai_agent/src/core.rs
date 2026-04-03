use crate::config::AgentConfig;
use crate::hooks::{AgentContext, AgentHooks};
use crate::llm::{LlmClient, LlmResponse};
use crate::mcp::McpManager;
use crate::tools::ToolDefinition;
use genai::chat::{ChatMessage, ToolResponse};
use std::error::Error;

/// Agent 结构体，聚合了配置、LLM客户端、MCP管理器、本地工具以及生命周期钩子
pub struct Agent<H: AgentHooks> {
    pub config: AgentConfig,
    pub llm_client: LlmClient,
    pub mcp_manager: McpManager,
    pub local_tools: Vec<ToolDefinition>,
    pub hooks: H,
    pub context: AgentContext,
    pub messages: Vec<ChatMessage>,
    pub initialized: bool,
}

impl<H: AgentHooks> Agent<H> {
    /// 创建一个新的 Agent 实例
    pub fn new(config: AgentConfig, hooks: H) -> Self {
        let llm_client = LlmClient::new(&config.model);
        Self {
            config,
            llm_client,
            mcp_manager: McpManager::new(),
            local_tools: Vec::new(),
            hooks,
            context: AgentContext::default(),
            messages: Vec::new(),
            initialized: false,
        }
    }


    /// 初始化 Agent
    pub async fn init(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.initialized {
            return Ok(());
        }

        self.hooks.on_init(&mut self.context).await?;

        if let Some(system_prompt) = &self.config.system_prompt {
            self.messages.push(ChatMessage::system(system_prompt));
        }

        self.mcp_manager
            .init_from_config(&self.config.mcp_servers)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        self.initialized = true;
        Ok(())
    }

    /// 运行核心的 Reasoning-Action Loop
    pub async fn run_loop(&mut self, user_input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        if !self.initialized {
            self.init().await?;
        }

        // 1. Input 阶段
        self.hooks.on_input(&mut self.context, user_input).await?;
        self.messages.push(ChatMessage::user(user_input));

        loop {
            // 2. Reasoning 阶段
            self.hooks.on_before_reasoning(&mut self.context).await?;

            // 提取所有注册的本地工具（后续可结合 MCP Tools）
            let tools = self.local_tools.clone();
            
            let tools_opt = if tools.is_empty() { None } else { Some(tools) };

            let response: LlmResponse = self
                .llm_client
                .chat(self.messages.clone(), tools_opt)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

            self.hooks.on_after_reasoning(&mut self.context).await?;

            // 保存 Assistant 的响应到历史消息中
            if !response.tool_calls.is_empty() {
                // 如果有工具调用，则保存工具调用记录
                self.messages.push(ChatMessage::from(response.tool_calls.clone()));
            } else if !response.content.is_empty() {
                // 否则保存纯文本回复
                self.messages.push(ChatMessage::assistant(&response.content));
            }

            // 3. Action 阶段
            if response.tool_calls.is_empty() {
                // 如果没有产生任何 Tool Call，则跳出循环
                self.hooks.on_after_loop(&mut self.context).await?;
                return Ok(response.content);
            }

            for tool_call in response.tool_calls {
                let tool_name = &tool_call.fn_name;
                let tool_args = tool_call.fn_arguments.to_string();

                self.hooks
                    .on_before_tool_execute(&mut self.context, tool_name, &tool_args)
                    .await?;

                // TODO: 这里应根据 tool_name 路由到具体的本地闭包或 MCP 执行接口
                // 目前模拟执行成功的结果
                let result = format!("Simulated execution for {}", tool_name);

                self.hooks
                    .on_after_tool_execute(&mut self.context, tool_name, &result)
                    .await?;

                // 将工具调用的结果加入消息队列
                let tool_response = ToolResponse::new(&tool_call.call_id, &result);
                self.messages.push(ChatMessage::from(tool_response));
            }

            self.hooks.on_after_loop(&mut self.context).await?;
        }
    }
}
