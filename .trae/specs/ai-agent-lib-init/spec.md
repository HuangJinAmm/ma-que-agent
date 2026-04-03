# AI Agent 基础库初始化 Spec

## Why
需要一个高性能、可扩展的基于 Rust 的 AI Agent 基础开发库，供其他项目作为 lib 引入和使用。目前缺乏统一的、原生支持最新 Claude 规范（Tools/Skills）及 MCP 协议的 Rust Agent 开发框架。

## What Changes
- 创建基于 Rust 的 `ai_agent` lib 库工程
- 引入 `genai` 库进行底层大模型的 API 交互与适配
- 引入 `serde`、`toml` 实现声明式配置
- 实现 Claude Tools 和 Skills 的结构兼容与执行路由映射
- 引入 `rmcp` Rust SDK 支持 MCP 客户端能力
- 设计并实现具有完整生命周期钩子（Hooks）的 Agent Loop 核心流转机制

## Impact
- Affected specs: 确立 AI Agent 的底层架构设计
- Affected code: 将从零构建核心模块：`config`, `core`, `llm`, `tools`, `mcp`, `hooks`

## ADDED Requirements
### Requirement: 核心基础结构
系统必须作为一个标准的 Rust Library (lib) 供外部使用，采用模块化设计。

### Requirement: 配置模块
系统必须支持通过 TOML 文件声明式地定义 Agent 的核心行为（如名称、模型、工具列表、MCP 配置等）。

### Requirement: 工具与技能规范
系统必须兼容 Claude Tools 和 Skills 规范，能够将 Rust 原生函数转换为 Claude Tools 规范的 JSON 定义。

### Requirement: MCP 集成
系统必须支持基于官方 `rmcp` SDK 构建 MCP Client，动态获取并挂载 MCP 资源、提示词和工具。

### Requirement: Agent Loop 与 Hooks
系统必须实现完整的推理-动作循环（Reasoning-Action Loop），并在关键生命周期节点暴露扩展接口：
- `on_init`
- `on_input`
- `on_before_reasoning`
- `on_after_reasoning`
- `on_before_tool_execute`
- `on_after_tool_execute`
- `on_after_loop`