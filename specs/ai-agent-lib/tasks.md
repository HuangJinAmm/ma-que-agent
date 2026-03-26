# AI Agent Rust Lib 实施任务拆解

## 1. 任务目标

将规格说明拆解为可执行的工程任务，指导后续以 Rust `lib` 形式交付一个可配置、可扩展、可观测的 AI Agent 基础库。

## 2. 里程碑

### M1 基础骨架可运行

- 完成 crate 初始化与模块边界搭建
- 完成统一错误模型与基础配置加载
- 完成 `genai` 模型封装与最小 agent 执行链路

### M2 核心能力可配置

- 完成 tools registry
- 完成 agent / sub agent 配置驱动运行
- 完成 agent 记忆能力
- 完成上下文管理、token 治理、限流与重试

### M3 生态集成可用

- 完成 MCP 集成
- 完成 Claude Skills 集成
- 完成 workflow 引擎

### M4 多 agent 互联可用

- 完成 A2A client / server
- 完成完整观测与集成测试
- 发布首个可复用版本

## 3. 任务拆解

### T1 工程初始化与模块边界

**目标**

建立 Rust 库工程、目录结构、公共 API 和基础依赖。

**交付物**

- `Cargo.toml`
- `src/lib.rs`
- 一级模块骨架
- 特性开关策略

**子任务**

- 初始化 `lib` crate
- 建立 `config`、`agent`、`model`、`context`、`memory`、`tooling`、`mcp`、`skills`、`a2a`、`workflow`、`runtime`、`observability`、`error` 模块
- 设计 `prelude` 与公共导出
- 约定 feature flags，如 `mcp`、`a2a`、`skills`、`workflow`

**完成标准**

- 项目可编译
- 模块边界清晰
- 公共 API 暂无循环依赖

### T2 配置系统

**目标**

实现 TOML 配置解析、校验、合并与环境变量插值，并覆盖记忆相关配置。

**交付物**

- `AgentSpec`、`SubAgentSpec`、`ToolSpec`、`McpServerSpec`、`WorkflowSpec`、`MemoryStoreSpec`
- 配置加载器
- 配置校验器

**子任务**

- 定义 serde 数据结构
- 支持单文件与目录聚合
- 支持默认值继承与局部覆盖
- 支持环境变量插值
- 支持记忆 store、记忆策略与作用域配置
- 输出带路径定位的错误信息

**完成标准**

- 示例 TOML 可正确加载
- 非法字段与缺失字段可明确报错
- agent / tool / workflow 引用关系可校验

### T3 统一模型层

**目标**

基于 `genai` 封装统一模型调用接口。

**交付物**

- `ModelBackend` trait
- `GenaiBackend` 实现
- 请求与响应转换器

**子任务**

- 封装 `ClientBuilder`、`ClientConfig`
- 实现 chat 请求、stream 请求
- 映射 tool calling 能力
- 输出 token 使用量与模型元数据

**完成标准**

- 至少一条 chat 调用链路可运行
- 支持同步结果和流式结果
- 工具调用结果可回注会话

### T4 Agent 与 Sub Agent 执行引擎

**目标**

实现主 agent 执行、sub agent 委派和能力路由。

**交付物**

- `AgentRuntime`
- `AgentHandle`
- `SubAgentRouter`
- 会话状态容器

**子任务**

- 构建 agent 实例工厂
- 支持按名称获取 agent
- 实现 sub agent 路由接口
- 接入 agent 级记忆作用域与共享记忆作用域
- 限制递归深度与最大委派次数
- 统一执行上下文与返回结果模型

**完成标准**

- 主 agent 可调用 sub agent
- 子 agent 可继承并覆盖策略
- 路由失败有明确错误语义

### T5 Tools 注册中心

**目标**

建立统一工具抽象与注册管理能力。

**交付物**

- `Tool` trait
- `ToolRegistry`
- tool descriptor 与 schema 结构

**子任务**

- 定义工具元数据与输入输出约束
- 支持静态注册和运行时注册
- 支持原生 tool 与代理 tool
- 增加命名空间和冲突检测
- 增加工具执行日志与耗时统计

**完成标准**

- agent 可按可见范围拿到 tools 列表
- 工具可并发受控执行
- 工具失败能被分类与追踪

### T6 MCP 集成层

**目标**

通过配置接入 MCP server 并映射为工具。

**交付物**

- MCP transport 抽象
- stdio server 管理器
- MCP tool 适配器

**子任务**

- 定义 MCP server 配置结构
- 管理 server 生命周期
- 获取远端 tools 描述并缓存
- 将 MCP tool 映射为本地 `ToolDescriptor`
- 处理超时、断线与重连

**完成标准**

- 能接入至少一个 stdio MCP server
- MCP tools 可被 agent 调用
- server 故障能被隔离并上报

### T7 Claude Skills 集成

**目标**

兼容 Claude Skills 目录和元数据规范。

**交付物**

- `SkillProvider`
- skill 扫描器
- skill 运行适配器

**子任务**

- 定义 skills 根目录与路径校验规则
- 读取 skill 元数据
- 构建 skill 检索索引
- 将 skill 转换为 agent 可消费的执行单元
- 设计 skill 与 tool / prompt 注入关系

**完成标准**

- 指定目录内 skills 可被发现
- skill 可按名称启用
- skill 结果可写回会话上下文

### T8 上下文治理与 Token 管理

**目标**

实现 token 预算、历史裁剪和压缩。

**交付物**

- `ContextManager`
- `TokenBudget`
- `ContextCompressor`

**子任务**

- 设计结构化消息存储
- 实现估算 token 的统一接口
- 实现窗口裁剪、重要性裁剪
- 实现摘要压缩与工具结果折叠
- 保留压缩前后追踪元数据

**完成标准**

- 超预算时系统优先压缩而非立即失败
- 压缩后仍保留任务所需关键上下文
- token 使用量可在结果中观测

### T9 Agent 记忆系统

**目标**

实现跨轮次、跨会话可控的 agent 记忆能力。

**交付物**

- `MemoryStore`
- `MemoryRetriever`
- `MemoryWriter`
- 记忆抽取与注入管线

**子任务**

- 定义记忆类型、作用域、元数据与查询模型
- 实现会话记忆、情节记忆、语义记忆、偏好记忆抽象
- 实现自动写入、显式写入和禁用写入模式
- 实现去重、TTL、重要性评分、来源审计
- 实现关键词检索、标签过滤与混合召回
- 将召回结果纳入 prompt 注入预算控制
- 为文件型或 SQLite 型 backend 提供首版实现

**完成标准**

- agent 可按策略写入和检索记忆
- 记忆支持作用域隔离与显式删除
- 召回结果可稳定注入上下文且受 token 预算约束

### T10 限流、重试与稳定性治理

**目标**

实现模型、agent、tool 的统一治理中间件。

**交付物**

- 限流器
- 重试器
- 超时与取消传播机制

**子任务**

- 设计统一 `ExecutionPolicy`
- 实现指数退避
- 区分可重试与不可重试错误
- 增加熔断与并发上限
- 接入 tracing 与 metrics

**完成标准**

- 高并发下可按配置限流
- 临时失败可自动重试
- 长耗时请求可超时退出并释放资源

### T11 Workflow 引擎

**目标**

实现基于 TOML 的工作流定义与执行。

**交付物**

- `WorkflowEngine`
- 节点执行器
- 变量映射机制

**子任务**

- 定义 workflow schema
- 实现 agent、tool、skill、MCP、A2A、branch、parallel、join、end 节点
- 实现 DAG 校验
- 实现步骤级重试、超时和补偿
- 提供执行轨迹输出

**完成标准**

- 示例 workflow 可成功执行
- 并行与分支逻辑可验证
- 非法图结构可在加载时阻止

### T12 A2A Client / Server

**目标**

实现 A2A 协议基础互通能力。

**交付物**

- A2A 数据模型
- A2A client
- A2A server
- Agent Card 暴露接口

**子任务**

- 定义 task、message、artifact、status 模型
- 实现 HTTP + JSON-RPC 2.0 client
- 实现 SSE 订阅
- 实现 server 端路由与 task 生命周期管理
- 统一远端错误映射

**完成标准**

- 能读取远端 Agent Card
- 能发起任务并获取结果或流式状态
- 本地 agent 可通过 server 对外暴露

### T13 可观测性与审计

**目标**

提供生产可观测性基础设施。

**交付物**

- tracing span 设计
- metrics 指标
- 审计事件模型

**子任务**

- 为核心链路增加 trace_id
- 暴露 token、重试、限流、工具失败等指标
- 暴露记忆写入、召回命中、去重和注入预算指标
- 为敏感字段增加脱敏输出
- 为 workflow / A2A / MCP 增加审计事件

**完成标准**

- 核心执行链路可追踪
- 关键错误与耗时可定位
- 日志不泄露敏感信息

### T14 测试与示例

**目标**

建立单元测试、集成测试和示例工程。

**交付物**

- 单元测试
- 集成测试
- 示例配置与示例调用代码

**子任务**

- 为配置、tooling、context、memory、workflow、A2A 编写测试
- 构建 mock model backend
- 构建最小 MCP 测试桩
- 编写完整示例 `agent.toml`
- 编写 README 使用示例

**完成标准**

- 核心模块具备稳定测试覆盖
- 示例工程可跑通主流程
- 对外文档足够支撑接入

## 4. 建议实施顺序

1. T1 工程初始化与模块边界
2. T2 配置系统
3. T3 统一模型层
4. T4 Agent 与 Sub Agent 执行引擎
5. T5 Tools 注册中心
6. T8 上下文治理与 Token 管理
7. T9 Agent 记忆系统
8. T10 限流、重试与稳定性治理
9. T6 MCP 集成层
10. T7 Claude Skills 集成
11. T11 Workflow 引擎
12. T12 A2A Client / Server
13. T13 可观测性与审计
14. T14 测试与示例

## 5. 风险与对策

### 风险 1：`genai` 能力边界与目标抽象不完全一致

**对策**

- 将模型层封装在独立模块
- 对工具调用与流式能力建立适配层
- 对后续替换或补充 provider 保留扩展接口

### 风险 2：MCP 与 A2A 都涉及外部协议，集成复杂度高

**对策**

- 先做协议抽象，再落地最小可用 transport
- 首版只强制交付最小必需能力
- 对高级能力预留接口而不强行一次做全

### 风险 3：上下文压缩效果不稳定

**对策**

- 支持多种压缩策略
- 为压缩过程保留元数据
- 用回归测试验证关键任务质量

### 风险 4：多 agent 调度易出现递归失控

**对策**

- 限制最大深度、最大委派次数、最大 token 消耗
- 增加运行时保护与错误回退

### 风险 5：长期记忆污染或召回噪声影响回答质量

**对策**

- 对记忆写入增加阈值、去重和来源审计
- 对召回增加作用域过滤、分数阈值和 token 预算限制
- 用离线评测与回归测试验证记忆命中质量

## 6. Definition of Done

- 所有核心模块均有稳定公共 API
- 示例配置可完整驱动主 agent、sub agent、tools、MCP、workflow
- `genai` 链路、上下文治理、记忆系统、限流重试、技能加载、A2A 基础链路可运行
- 关键模块有自动化测试
- 对外文档可指导其他 Rust 项目作为库接入
