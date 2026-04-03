use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// MCP Server 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// 启动命令 (如 "npx", "python")
    pub command: String,
    /// 命令行参数
    #[serde(default)]
    pub args: Vec<String>,
}

/// Agent 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent 的名称
    pub name: String,
    /// 使用的 LLM 模型名称 (例如 "gpt-4o", "claude-3-5-sonnet-20241022")
    pub model: String,
    /// 系统提示词 (System Prompt)
    pub system_prompt: Option<String>,
    /// 绑定的工具或技能名称列表
    #[serde(default)]
    pub tools: Vec<String>,
    /// MCP Servers 配置，键为 MCP Server 的名称
    #[serde(default)]
    pub mcp_servers: std::collections::HashMap<String, McpServerConfig>,
    /// LLM 采样温度
    pub temperature: Option<f32>,
    /// 最大生成的 token 数量
    pub max_tokens: Option<u32>,
}

impl AgentConfig {
    /// 从 TOML 文件中加载 Agent 配置
    ///
    /// # 参数
    ///
    /// * `path` - TOML 配置文件的路径
    ///
    /// # 错误
    ///
    /// 如果文件读取失败或反序列化失败，则返回错误。
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: AgentConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_from_toml() {
        let toml_content = r#"
            name = "TestAgent"
            model = "gpt-4o"
            system_prompt = "You are a helpful assistant."
            tools = ["search", "calculator"]
            temperature = 0.7
            max_tokens = 1000
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", toml_content).unwrap();

        let config = AgentConfig::load_from_toml(temp_file.path()).unwrap();
        
        assert_eq!(config.name, "TestAgent");
        assert_eq!(config.model, "gpt-4o");
        assert_eq!(config.system_prompt.unwrap(), "You are a helpful assistant.");
        assert_eq!(config.tools, vec!["search", "calculator"]);
        assert_eq!(config.temperature, Some(0.7));
        assert_eq!(config.max_tokens, Some(1000));
    }
}
