# AI Agent Rust Lib 规格说明

## 1. 文档目的

本文档定义一个基于 Rust 的 AI Agent 通用库规格。该库以 `lib` 形态对外提供能力，供 CLI、服务端、桌面端或其他 Rust 项目嵌入使用。核心模型接入统一基于 [`genai`](https://crates.io/crates/genai) 实现，并围绕 agent 配置、工具系统、MCP、Claude Skills、A2A 协议、上下文治理与工作流编排构建完整能力层。

## 2. 目标与边界

### 2.1 目标

- 提供可复用、可嵌入、可扩展的 Rust Agent 库。
- 支持通过 TOML 描述 agent、sub agent、tools、MCP、workflow 等配置。
- 统一封装 `genai`，向上暴露稳定的会话、工具调用、上下文管理和多 agent 协作接口。
- 兼容 Claude Skills 规范，实现 skills 的加载、发现、选择和执行编排。
- 支持 A2A 协议，使本地 agent 可作为 A2A client / server 参与跨系统协作。
- 提供生产可用的配额治理能力，包括 token 限制、上下文压缩、限流、重试、熔断与观测。

### 2.2 非目标

- 不实现面向终端用户的 UI。
- 不绑定单一运行时形态，不直接限定为 CLI 或 HTTP 服务。
- 不内置具体业务 tools，仅提供注册、发现、执行与治理框架。
- 不把 MCP server、A2A server 强耦合为进程内实现，优先支持可插拔接入。

## 3. 总体设计原则

- 库优先：所有能力以 crate API 暴露，二进制入口可作为后续示例工程单独实现。
- 配置驱动：运行时行为优先由 TOML 配置决定，代码作为扩展点。
- 提供者解耦：模型层通过 `genai` 统一适配不同大模型提供方，避免供应商锁定。
- 强扩展性：agent、tool、skill、MCP、workflow、A2A transport 均采用 trait + registry 模式。
- 强治理：把 token、上下文、调用配额、超时、重试、审计作为一等能力。
- 安全默认：默认最小权限、默认超时、默认输入校验、默认敏感信息脱敏。

## 4. 用户场景

### 4.1 典型使用者

- 需要在现有 Rust 项目中嵌入 agent 能力的应用开发者。
- 需要构建多 agent 协作系统的平台开发者。
- 需要对接 MCP 工具生态和 A2A 互联能力的基础设施团队。

### 4.2 典型场景

- 在业务服务中加载一个主 agent，并按任务路由到多个 sub agent。
- 使用 TOML 配置外部 MCP server 与内置 tools，共同参与工具调用。
- 根据 Claude Skills 描述动态装载 skill 并让 agent 选择执行。
- 当上下文过长时自动压缩历史、摘要旧消息，并遵守 token 预算。
- 通过 A2A 向外部 agent 发起任务，或作为 A2A server 对外暴露自身能力。
- 使用 workflow 将多步任务拆解为串行、并行、条件分支和回滚流程。

## 5. 功能需求

### 5.1 TOML 配置定义 agent 与 sub agent

系统必须支持通过 TOML 文件声明以下对象：

- 全局运行配置
- 模型提供者与默认模型配置
- 记忆存储与索引配置
- 主 agent 定义
- sub agent 定义
- skills 配置
- MCP 配置
- tools 配置
- workflow 配置
- 限流、重试、token、上下文策略配置

#### 5.1.1 Agent 配置能力

每个 agent 至少支持以下字段：

- `name`
- `description`
- `system_prompt`
- `model`
- `temperature`
- `max_tokens`
- `tools`
- `skills`
- `mcp_servers`
- `sub_agents`
- `memory_policy`
- `memory_scope`
- `memory_retrieval_policy`
- `memory_write_policy`
- `context_policy`
- `rate_limit_policy`
- `retry_policy`
- `workflow_entry`
- `metadata`

#### 5.1.2 Sub Agent 配置能力

sub agent 必须支持：

- 被主 agent 通过名称、标签或能力路由
- 继承全局默认配置并允许局部覆盖
- 声明专属 tools / skills / MCP server
- 声明最大递归调用深度与最大委派次数
- 声明是否允许被其他 sub agent 再次调用

#### 5.1.3 配置加载要求

- 支持单文件与目录合并加载。
- 支持环境变量插值。
- 支持 schema 校验与错误定位。
- 支持热重载能力扩展点，但首版可只要求显式重载。

### 5.2 MCP 支持

系统必须支持通过 TOML 声明 MCP server，并统一纳入 tool 生态。

#### 5.2.1 MCP 配置内容

- server 名称
- 启动方式
- transport 类型
- command / args / env
- 超时
- 自动重连策略
- 能力白名单
- 是否暴露为 agent tools
- 认证与安全策略

#### 5.2.2 MCP 运行要求

- 支持 stdio 类型 MCP server。
- 为后续 streamable HTTP / SSE 类型预留 transport 抽象。
- 支持 server 生命周期管理、心跳、重连、关闭。
- 支持工具发现缓存与失效刷新。
- 支持将 MCP tool 映射为统一 `ToolDescriptor`。

### 5.3 Claude Skills 规范支持

系统必须兼容 Claude Skills 的目录组织和元数据约定，至少支持：

- skills 根目录发现
- skill 元数据读取
- skill 描述、输入约束和使用说明解析
- 按名称或语义标签查找 skill
- 将 skill 暴露为 agent 可调用能力
- skill 执行前后的上下文注入策略

#### 5.3.1 Skills 运行模型

- skill 可被建模为一种高层能力单元，而非简单函数。
- skill 可声明依赖的 tools、额外 prompt 片段和执行前置条件。
- skill 的执行结果可写回当前会话上下文。
- skill 必须支持权限与来源校验，避免任意目录注入。

### 5.4 Tools 注册与管理

系统必须提供统一的工具注册中心。

#### 5.4.1 Tool 抽象

每个 tool 必须具备：

- 唯一名称
- 描述
- 输入 schema
- 输出 schema 或输出约束
- 执行器
- 超时配置
- 并发限制
- 权限标签
- 可观测性标签

#### 5.4.2 Tool 管理能力

- 支持静态注册和运行时注册。
- 支持内置 tool、本地自定义 tool、MCP tool 三类来源。
- 支持冲突检测、命名空间隔离与版本标识。
- 支持按 agent 范围过滤可见 tools。
- 支持工具调用日志、耗时统计、失败分类。

### 5.5 A2A 协议支持

系统必须支持 A2A 协议基础能力，以实现 agent 对 agent 互通。

#### 5.5.1 目标能力

- 作为 A2A client 发起远程任务
- 作为 A2A server 暴露本地 agent 能力
- 支持 Agent Card 暴露与读取
- 支持同步请求响应
- 支持长任务状态跟踪
- 支持 SSE 流式更新
- 为 webhook 推送回调预留接口

#### 5.5.2 协议抽象要求

- 传输层优先支持 HTTP(S) + JSON-RPC 2.0。
- Server 端能力模型需包含 agent card、task、artifact、message、status。
- Client 端需提供发现、调用、取消、轮询、流式订阅能力。
- 错误模型需统一映射为库内错误类型。

### 5.6 Token 限制、上下文管理与治理

系统必须提供完整的上下文治理能力。

#### 5.6.1 Token 预算

- 支持按全局、agent、会话、单次请求配置最大 token。
- 支持输入预算、输出预算、保留预算分离。
- 支持依据模型能力动态选择预算策略。

#### 5.6.2 上下文管理

- 维护结构化消息历史。
- 支持窗口裁剪、按重要性裁剪、按角色裁剪。
- 支持摘要压缩、记忆提炼、工具结果折叠。
- 支持将系统提示、skills 注入、tool 结果、A2A 结果统一纳入上下文管线。

#### 5.6.3 压缩策略

- 超预算时优先压缩历史，而非直接失败。
- 压缩至少支持：截断、摘要、结构化提炼三种策略。
- 压缩结果需保留来源信息和可追踪元数据。

#### 5.6.4 限流、重试与稳定性

- 支持按 agent、tool、模型提供方配置限流器。
- 支持指数退避重试与最大重试次数。
- 支持针对可重试错误和不可重试错误的分类策略。
- 支持超时、断路器、幂等键与取消传播。

### 5.7 Agent 记忆功能

系统必须支持 agent 记忆能力，并将其作为上下文治理之外的独立能力域设计。

#### 5.7.1 记忆类型

至少支持以下记忆类型：

- 会话记忆：当前会话内短期有效的任务事实、用户约束、工具结果摘要
- 情节记忆：跨会话保留的重要事件、任务结果和执行经验
- 语义记忆：经过提炼的稳定知识、事实、术语、实体关系
- 偏好记忆：用户习惯、输出风格、禁忌项、长期偏好

#### 5.7.2 记忆作用域

至少支持以下作用域：

- `run`：单次运行有效
- `session`：单会话有效
- `agent`：某个 agent 私有
- `shared`：多个 agent 共享
- `project`：项目级共享
- `user`：按用户维度隔离

#### 5.7.3 记忆生命周期

记忆流程至少包括：

- 写入前抽取：从消息、工具结果、workflow 输出、A2A 结果中抽取候选记忆
- 归一化与分类：确定类型、作用域、标签、重要性、来源
- 去重与合并：避免重复写入和相似内容膨胀
- 持久化：按配置写入对应 backend
- 检索与注入：在模型调用前按策略召回并注入上下文
- 衰减与清理：支持 TTL、容量上限、重要性衰减和显式删除

#### 5.7.4 记忆写入策略

- 支持自动写入、显式写入、关闭写入三种模式。
- 支持按消息角色、tool 来源、workflow 节点类型过滤可写入内容。
- 支持最低重要性阈值、最大写入频率、批量写入窗口。
- 支持人工确认型记忆预留接口，用于高风险长期记忆写入。

#### 5.7.5 记忆检索策略

- 支持按关键词、标签、结构化过滤条件检索。
- 支持按语义相似度召回长期记忆。
- 支持按最近性、重要性、相似度的混合排序。
- 支持 `top_k`、最小分数阈值、最大注入 token 预算。
- 支持在主 agent、sub agent、workflow 节点执行前进行差异化召回。

#### 5.7.6 存储与索引要求

- 至少提供内存型 backend。
- 首版建议提供文件型或 SQLite 型持久化 backend。
- 为向量检索、外部 KV、外部数据库预留 `MemoryStore` 扩展接口。
- 支持记忆元数据索引与可选语义索引。
- 语义记忆可通过 `genai` embeddings 或可替换 embedding backend 建立索引。

#### 5.7.7 与上下文系统的关系

- 记忆系统不等价于消息历史。
- 上下文管理负责窗口裁剪与压缩，记忆系统负责跨窗口、跨会话保留与召回。
- 历史压缩结果可作为候选记忆输入。
- 召回记忆进入 prompt 前必须经过格式化和 token 预算控制。

#### 5.7.8 安全与治理要求

- 支持按作用域隔离记忆，避免串写和越权读取。
- 支持敏感信息标记、脱敏存储和禁止持久化策略。
- 支持记忆来源审计，包括写入来源、时间、触发链路和关联会话。
- 支持记忆失效、删除、重建和批量迁移接口。

### 5.8 工作流配置

系统必须支持通过 TOML 定义 workflow。

#### 5.8.1 工作流节点

至少支持以下节点类型：

- agent 调用
- sub agent 委派
- tool 调用
- skill 执行
- MCP tool 调用
- A2A 远程任务
- 条件分支
- 并行分支
- 汇聚节点
- 结束节点

#### 5.8.2 工作流能力

- 支持 DAG 风格定义。
- 支持输入输出变量映射。
- 支持步骤级超时、重试和补偿策略。
- 支持工作流级 trace id 与审计记录。
- 支持嵌套 workflow，但需限制最大嵌套深度。

## 6. 架构设计

### 6.1 crate 分层

建议采用单 crate 多模块方案，后续如复杂度上升可拆分 workspace：

- `config`：TOML 解析、schema 校验、环境变量插值
- `agent`：Agent 定义、会话执行、sub agent 路由
- `model`：`genai` 客户端封装、模型选择、请求转换
- `context`：消息存储、token 估算、裁剪与压缩
- `memory`：记忆抽取、存储、检索、索引、生命周期管理
- `tooling`：tool trait、registry、执行治理
- `mcp`：MCP server 管理、协议适配、tool 映射
- `skills`：Claude Skills 发现、加载、选择、执行
- `a2a`：A2A client/server、协议模型、传输抽象
- `workflow`：workflow 解析、编排与执行引擎
- `runtime`：限流、重试、超时、并发调度、事件总线
- `observability`：日志、metrics、tracing、审计事件
- `error`：统一错误模型

### 6.2 核心 trait

建议定义如下抽象：

- `AgentExecutor`
- `SubAgentRouter`
- `ModelBackend`
- `ContextManager`
- `ContextCompressor`
- `MemoryStore`
- `MemoryIndexer`
- `MemoryRetriever`
- `MemoryWriter`
- `Tool`
- `ToolRegistry`
- `SkillProvider`
- `McpTransport`
- `McpServerHandle`
- `A2aClient`
- `A2aServer`
- `WorkflowEngine`

### 6.3 关键数据模型

- `AgentSpec`
- `SubAgentSpec`
- `ToolSpec`
- `McpServerSpec`
- `SkillSpec`
- `WorkflowSpec`
- `SessionState`
- `ContextWindow`
- `MemoryRecord`
- `MemoryQuery`
- `MemoryPolicy`
- `MemoryScope`
- `TokenBudget`
- `ExecutionPolicy`
- `RetryPolicy`
- `RateLimitPolicy`
- `A2aTaskRef`

## 7. 配置格式示例

```toml
[runtime]
default_agent = "orchestrator"
skills_dir = "./skills"
max_concurrent_tasks = 32

[memory]
default_store = "local"
default_scope = "project"

[memory.stores.local]
kind = "sqlite"
path = "./.agent/memory.db"

[memory.policies.default]
write_mode = "auto"
retrieval = "hybrid"
top_k = 8
max_injection_tokens = 1200
dedup = true
ttl_days = 30

[providers.default]
model = "gpt-4o-mini"
temperature = 0.2
max_tokens = 4096

[policies.token]
max_input_tokens = 24000
max_output_tokens = 4000
reserve_tokens = 2000
compression = "summarize"

[policies.retry]
max_attempts = 3
backoff = "exponential"

[policies.rate_limit]
permits_per_second = 5
burst = 10

[mcp.servers.filesystem]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "."]
expose_as_tools = true
timeout_ms = 10000

[tools.search_docs]
kind = "native"
description = "搜索内部知识库"
input_schema = "schemas/search_docs.json"
timeout_ms = 5000

[skills.code_review]
source = "./skills/code_review"
enabled = true

[agents.orchestrator]
description = "主调度 agent"
system_prompt = "你负责规划、路由与汇总"
model = "gpt-4o-mini"
tools = ["search_docs"]
skills = ["code_review"]
mcp_servers = ["filesystem"]
sub_agents = ["researcher", "writer"]
memory_scope = "project"
memory_retrieval_policy = "default"
memory_write_policy = "default"
workflow_entry = "default_flow"

[agents.researcher]
description = "负责搜索与归纳"
system_prompt = "专注信息检索与结构化总结"
model = "claude-3-5-sonnet"
tools = ["search_docs"]
memory_scope = "agent"

[agents.writer]
description = "负责内容生成"
system_prompt = "专注生成可交付内容"
model = "gpt-4o-mini"
memory_scope = "project"

[workflows.default_flow]
entry = "plan"

[[workflows.default_flow.nodes]]
id = "plan"
type = "agent"
agent = "orchestrator"
next = ["delegate"]

[[workflows.default_flow.nodes]]
id = "delegate"
type = "parallel"
next = ["merge"]

[[workflows.default_flow.nodes]]
id = "merge"
type = "join"
next = ["done"]

[[workflows.default_flow.nodes]]
id = "done"
type = "end"
```

## 8. API 设计要求

### 8.1 对外入口

库至少提供以下入口能力：

- 从路径加载配置并构建运行时
- 通过代码手动注册 tools / skills / providers
- 创建会话并执行 agent
- 查询、写入和清理 agent 记忆
- 触发 workflow 执行
- 调用 A2A 远程 agent
- 启动 A2A server

### 8.2 建议 API 形态

```rust
let runtime = AgentRuntime::builder()
    .with_config_path("agent.toml")
    .register_tool(my_tool)
    .build()
    .await?;

let result = runtime
    .agent("orchestrator")?
    .run("请分析这个仓库并给出重构建议")
    .await?;

let memories = runtime
    .memory()
    .search("用户偏好与项目约束", "project")
    .await?;
```

### 8.3 错误语义

错误类型需至少覆盖：

- 配置错误
- 模型调用错误
- tool 执行错误
- MCP 通信错误
- 记忆写入错误
- 记忆检索错误
- 记忆存储错误
- skill 加载错误
- A2A 协议错误
- workflow 执行错误
- token 超限错误
- 限流错误
- 超时错误

## 9. 非功能需求

### 9.1 性能

- 单次会话执行路径应避免不必要的深拷贝。
- 配置、tool 描述、skill 元数据应支持缓存。
- 长会话场景下上下文裁剪需具备稳定性能。
- 记忆检索与注入需有独立性能预算，避免拖慢主执行链路。

### 9.2 可测试性

- 核心模块需支持 mock。
- `genai` 访问层需可替换为测试 backend。
- workflow、tooling、context、A2A 协议层需具备单元测试与集成测试入口。
- 记忆抽取、召回排序、TTL 和去重逻辑需具备可重复测试。

### 9.3 可观测性

- 暴露 tracing span。
- 提供调用链 trace_id。
- 提供模型、tools、MCP、A2A、workflow 执行事件。
- 提供 token 使用量、重试次数、限流命中率等指标。
- 提供记忆写入量、召回命中率、去重率、记忆注入 token 占比等指标。

### 9.4 安全

- 凭证不得写入日志。
- tool / MCP / skill 加载需进行路径与权限校验。
- A2A 与 MCP 的外部输入必须做 schema 校验与长度限制。
- 默认启用超时和并发上限。
- 敏感记忆默认不得持久化，需支持作用域隔离与删除。

## 10. 兼容性要求

### 10.1 Rust 版本

- 目标 MSRV 建议为稳定 Rust 的当前企业常用版本，首版建议不少于 `1.80`。

### 10.2 平台

- 至少支持 Linux、macOS、Windows。

### 10.3 异步运行时

- 默认基于 `tokio`。
- 对外 API 以 async 为主。

## 11. 依赖建议

核心依赖建议如下：

- `genai`
- `tokio`
- `serde`
- `serde_json`
- `toml`
- `schemars`
- `thiserror`
- `tracing`
- `async-trait`
- `reqwest`
- `tower`
- `dashmap`
- `uuid`

说明：

- `genai` 用于统一模型接入与工具调用承载。
- `tower` 可用于限流、重试、超时等中间件治理。
- `schemars` 用于 tool 和配置 schema 描述。
- `uuid` 可用于会话、记忆和追踪对象标识。

## 12. 实施阶段建议

### 阶段一

- 完成配置系统
- 完成 `genai` 适配层
- 完成 agent / sub agent 执行骨架
- 完成原生 tools registry

### 阶段二

- 完成 agent 记忆系统
- 完成 MCP 接入
- 完成 Claude Skills 兼容
- 完成上下文治理与 token 预算

### 阶段三

- 完成 workflow 引擎
- 完成 A2A client/server
- 完成可观测性与稳定性增强

## 13. 验收标准

项目在首个可用版本中至少满足以下标准：

- 可通过 TOML 成功定义主 agent、sub agent、MCP、tools、workflow。
- 可通过 `genai` 发起模型请求并支持 tool calling。
- 可成功注册原生 tools 与 MCP tools。
- 可发现并装载 Claude Skills。
- 可按配置完成记忆写入、检索、注入与清理。
- 可在超出 token 预算时触发上下文压缩。
- 可按配置执行限流和重试。
- 可运行一个包含路由、工具调用和汇总的多 agent workflow。
- 可作为 A2A client 调用远端 agent，并可作为 A2A server 暴露本地能力。
