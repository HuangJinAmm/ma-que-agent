# Tasks

- [x] Task 1: 项目基础结构初始化
  - [x] SubTask 1.1: 创建 Rust lib 工程 `ai_agent`
  - [x] SubTask 1.2: 配置 `Cargo.toml` 引入基础依赖 (`genai`, `rmcp`, `serde`, `toml`, `tokio` 等)
  - [x] SubTask 1.3: 创建核心模块文件结构 (`config.rs`, `core.rs`, `llm.rs`, `tools.rs`, `mcp.rs`, `hooks.rs`)

- [x] Task 2: 配置解析模块实现 (`config` 模块)
  - [x] SubTask 2.1: 基于 TOML 定义 Agent 配置对应的数据结构 (如 `name`, `model`, `system_prompt`, `tools` 等)
  - [x] SubTask 2.2: 实现基于 `serde` 和 `toml` 的配置文件反序列化与加载逻辑

- [x] Task 3: 扩展接口（Hooks）定义 (`hooks` 模块)
  - [x] SubTask 3.1: 定义完整的生命周期 Hooks Trait (`on_init`, `on_input`, `on_before_reasoning`, `on_after_reasoning`, `on_before_tool_execute`, `on_after_tool_execute`, `on_after_loop`)

- [x] Task 4: LLM 交互与 Tools/Skills 规范 (`llm` 和 `tools` 模块)
  - [x] SubTask 4.1: 封装基于 `genai` 的底层请求客户端接口
  - [x] SubTask 4.2: 定义兼容 Claude Tools 规范的数据结构与映射机制
  - [x] SubTask 4.3: 实现对基础原子 Tools 进行打包的高内聚 Skill 集合逻辑

- [x] Task 5: MCP 协议集成 (`mcp` 模块)
  - [x] SubTask 5.1: 基于 `rmcp` Rust SDK 实现 MCP Client 生命周期管理与初始化逻辑
  - [x] SubTask 5.2: 实现 MCP Server 中 Resources, Prompts, Tools 的动态获取与注入到 Agent 上下文的能力

- [x] Task 6: 核心 Agent Loop 实现 (`core` 模块)
  - [x] SubTask 6.1: 定义 Agent 主结构体，整合 LLM, MCP, Tools 与 Hooks
  - [x] SubTask 6.2: 实现自主推理-动作循环 (Reasoning-Action Loop)，并在每个关键流转节点正确触发 Hooks 逻辑

# Task Dependencies
- [Task 2] depends on [Task 1]
- [Task 3] depends on [Task 1]
- [Task 4] depends on [Task 1]
- [Task 5] depends on [Task 1]
- [Task 6] depends on [Task 2], [Task 3], [Task 4], [Task 5]