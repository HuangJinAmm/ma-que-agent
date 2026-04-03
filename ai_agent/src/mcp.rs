use rmcp::{
    model::{Prompt, Resource, Tool},
    serve_client,
    service::{ClientInitializeError, RoleClient, RunningService},
    transport::child_process::TokioChildProcess,
};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use crate::config::McpServerConfig;

/// MCP 客户端管理器，负责连接和交互
pub struct McpClient {
    /// 运行中的 MCP 客户端服务
    service: RunningService<RoleClient, ()>,
}

impl McpClient {
    /// 初始化并连接到指定的 MCP Server（基于子进程）
    pub async fn connect_command(command: &str, args: &[&str]) -> Result<Self, ClientInitializeError> {
        let mut cmd = Command::new(command);
        for arg in args {
            cmd.arg(arg);
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());

        let process = TokioChildProcess::new(cmd).map_err(|e| {
            ClientInitializeError::transport::<TokioChildProcess>(e, "Failed to spawn MCP server process")
        })?;

        let service = serve_client((), process).await?;

        Ok(Self { service })
    }

    /// 获取所有的 Tools
    pub async fn list_tools(&self) -> Result<Vec<Tool>, rmcp::service::ServiceError> {
        self.service.peer().list_all_tools().await
    }

    /// 获取所有的 Resources
    pub async fn list_resources(&self) -> Result<Vec<Resource>, rmcp::service::ServiceError> {
        self.service.peer().list_all_resources().await
    }

    /// 获取所有的 Prompts
    pub async fn list_prompts(&self) -> Result<Vec<Prompt>, rmcp::service::ServiceError> {
        self.service.peer().list_all_prompts().await
    }
}

/// MCP 管理器，用于统一管理多个 MCP Client 的生命周期与能力
#[derive(Default)]
pub struct McpManager {
    clients: HashMap<String, McpClient>,
}

impl McpManager {
    /// 创建一个新的 MCP 管理器
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// 根据配置初始化多个 MCP Server 客户端
    pub async fn init_from_config(
        &mut self,
        configs: &HashMap<String, McpServerConfig>,
    ) -> Result<(), ClientInitializeError> {
        for (name, config) in configs {
            let args: Vec<&str> = config.args.iter().map(|s| s.as_str()).collect();
            let client = McpClient::connect_command(&config.command, &args).await?;
            self.clients.insert(name.clone(), client);
        }
        Ok(())
    }

    /// 获取所有挂载的 Tools 列表（聚合多个 MCP Server）
    pub async fn list_all_tools(&self) -> Result<Vec<Tool>, rmcp::service::ServiceError> {
        let mut all_tools = Vec::new();
        for client in self.clients.values() {
            let tools = client.list_tools().await?;
            all_tools.extend(tools);
        }
        Ok(all_tools)
    }

    /// 获取所有挂载的 Resources 列表（聚合多个 MCP Server）
    pub async fn list_all_resources(&self) -> Result<Vec<Resource>, rmcp::service::ServiceError> {
        let mut all_resources = Vec::new();
        for client in self.clients.values() {
            let resources = client.list_resources().await?;
            all_resources.extend(resources);
        }
        Ok(all_resources)
    }

    /// 获取所有挂载的 Prompts 列表（聚合多个 MCP Server）
    pub async fn list_all_prompts(&self) -> Result<Vec<Prompt>, rmcp::service::ServiceError> {
        let mut all_prompts = Vec::new();
        for client in self.clients.values() {
            let prompts = client.list_prompts().await?;
            all_prompts.extend(prompts);
        }
        Ok(all_prompts)
    }

    /// 获取指定的 MCP 客户端引用
    pub fn get_client(&self, name: &str) -> Option<&McpClient> {
        self.clients.get(name)
    }
}
