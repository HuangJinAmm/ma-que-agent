# AI Agent Library 设计规范文档

## 1. 概述 (Overview)
本项目旨在提供一个高性能、可扩展的 AI Agent 基础开发库。
- **开发语言**：Rust
- **项目定位**：作为基础 Library 供其他 Rust 项目引入和使用。
- **核心驱动**：基于 `genai` 库 ([https://crates.io/crates/genai](https://crates.io/crates/genai)) 进行底层大模型的 API 交互与适配。

## 2. 核心功能规范 (Core Features)

### 2.1 基于 TOML 的 Agent 定义
提供一种声明式的方式配置 Agent。通过反序列化（基于 `serde` 和 `toml`），允许开发者在项目外置文件中定义 Agent 的核心行为。
- **支持字段**：名称 (name)、模型版本 (model)、系统提示词 (system_prompt)、基础温度 (temperature)、挂载的工具列表 (tools/skills) 以及 MCP Servers 配置。
- **优势**：实现代码与 Prompt/配置的分离，便于调试和热更新。

### 2.2 支持 Claude Tools 规范
- **数据结构兼容**：严格按照 Claude 的 Tool use 规范定义 `name`, `description`, 和基于 JSON Schema 的 `input_schema`。
- **Rust 侧映射**：提供过程宏（Proc-Macro）或 Trait 体系，使 Rust 原生函数能够直接转换为符合 Claude Tools 规范的 JSON 定义，并能自动解析 LLM 返回的 Tool Use 请求执行相应函数。

### 2.3 支持 Claude Skills 规范
- **技能复用**：对特定的原子级 Tools 进行打包，形成高内聚的“Skill”（技能）。
- **规范对接**：使系统不仅支持零散的 Tool，还支持加载结构化的 Skill 集合，以便在多个 Agent 之间共享复杂的操作逻辑。

### 2.4 深度集成 MCP 协议
- **SDK 依赖**：基于官方的 `rmcp` Rust SDK 进行底层通讯。
- **能力边界拓展**：支持以 Client 身份连接任意标准的 Model Context Protocol (MCP) Server。动态获取 MCP Server 提供的 Resources、Prompts 以及 Tools，并无缝转换注入到当前 Agent 的上下文中。

---

## 3. Agent Loop 规范与扩展接口设计

Agent 的核心在于其自主的推理-动作循环（Reasoning-Action Loop）。本项目采用基于**生命周期钩子 (Lifecycle Hooks) / 中间件 (Middleware)** 的设计，以支持高扩展性。

### 3.1 完整的 Agent Loop 流程
1. **初始化 (Init)**：加载 TOML 配置，初始化 `genai` 客户端，建立 MCP 连接。
2. **输入接收 (Input)**：接收用户输入并追加到当前对话上下文中。
3. **推理阶段 (Reasoning)**：调用 `genai` 请求 LLM（携带 System Prompt、历史对话、可用 Tools）。
4. **意图解析 (Parsing)**：解析 LLM 响应。如果是普通文本则输出；如果是 Tool Call 则进入工具执行流程。
5. **工具执行 (Action)**：在本地或通过 MCP 路由执行对应工具，获取结果。
6. **上下文更新与循环 (Looping)**：将 Tool Result 拼接到上下文中，再次触发推理阶段，直到 LLM 判定任务完成（无新的 Tool Call 返回）。

### 3.2 扩展接口点设计 (Extension Hooks)
在 Loop 的关键节点暴露异步 Trait 接口，用于实现各种辅助功能：

#### `on_init` (初始化后)
- **配置加载**：加载 TOML 配置文件，初始化 `genai` 客户端，建立 MCP 连接。
- **资源初始化**：加载 Agent 所需的资源，如Mcp Server 配置、工具列表、技能列表等。

#### `on_input` (输入接收后)
- **用户输入处理**：接收用户输入，追加到当前对话上下文中。
- **安全检查 (输入拦截)**：对用户的 prompt 进行敏感词/注入攻击扫描。

#### `on_before_reasoning` (请求 LLM 前)
- **Token 统计管理与压缩**：在此处计算当前上下文 Token 消耗。如果超出阈值，触发上下文压缩策略（如摘要总结、截断早期记忆）。
- **AI 记忆注入**：检索长期记忆向量库或本地文件，将相关背景知识注入到 System Prompt 或上下文中。
- **人工干预 (Human-in-the-loop)**：在需求不清楚，目标不清晰时，需要向人工确认补充信息，或者选择方向。

#### `on_after_reasoning` (LLM 响应后)
- **安全检查 (输出拦截)**：校验大模型的输出文本是否合规。
- **Token 消耗统计**：记录本次请求的 Prompt Tokens 和 Completion Tokens，用于计费或配额管理。

#### `on_before_tool_execute` (工具执行前)
- **人工干预 (Human-in-the-loop)**：对于高风险工具（如执行终端命令、修改数据库、发送邮件），在此处挂起 Loop，向前端或控制台抛出确认请求，等待人工授权后继续。
- **权限校验**：二次确认当前 Agent 是否具备执行该 Tool 的权限。

#### `on_after_tool_execute` (工具执行后)
- **AI 记忆提取**：如果工具执行结果包含关键状态变化（如创建了新文件、获取了重要配置），在此处触发记忆更新机制，将状态写入持久化记忆库。
- **结果过滤**：对工具返回的超长结果（如巨大的日志或网页 HTML）进行预处理和裁剪，避免撑爆下一次 LLM 请求的上下文。

#### `on_after_loop` (循环结束)

- **状态重置**：在每次循环结束时，重置 Agent 状态，准备下一次对话。
- **日志记录**：记录当前对话的统计信息（如 Token 消耗、工具调用次数等），用于分析和调试。

## 4. 架构模块划分参考
- `ai_agent::config`：TOML 解析模块。
- `ai_agent::core`：包含 Agent 结构体与主 Loop 引擎。
- `ai_agent::llm`：基于 `genai` 库的统一封装接口。
- `ai_agent::tools`：Claude Tools/Skills 规范的数据结构与宏支持。
- `ai_agent::mcp`：基于 `rmcp` 的客户端生命周期管理。
- `ai_agent::hooks`：扩展接口的 Trait 定义与默认实现（安全、记忆、Token管理等）。