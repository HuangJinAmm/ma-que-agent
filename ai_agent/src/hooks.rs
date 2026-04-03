use std::error::Error;

/// Agent 上下文，包含当前会话的状态和相关信息。
/// 后续可根据核心循环的需要进行扩展。
#[derive(Debug, Default)]
pub struct AgentContext {
    // 预留上下文数据字段
}

/// AgentHooks Trait 定义了 AI Agent 在其运行生命周期中可被拦截或扩展的关键节点。
/// 所有的方法默认返回 Ok(())，使用者可以选择性地覆盖它们。
#[allow(async_fn_in_trait)]
pub trait AgentHooks: Send + Sync {
    /// 在 Agent 初始化时调用
    async fn on_init(&self, _context: &mut AgentContext) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// 在 Agent 接收到用户输入时调用
    async fn on_input(
        &self,
        _context: &mut AgentContext,
        _input: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// 在 Agent 开始推理前调用（例如：在发送请求给 LLM 之前）
    async fn on_before_reasoning(
        &self,
        _context: &mut AgentContext,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// 在 Agent 完成推理后调用（例如：收到 LLM 响应后）
    async fn on_after_reasoning(
        &self,
        _context: &mut AgentContext,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// 在 Agent 执行具体工具（Tool/Skill）前调用
    async fn on_before_tool_execute(
        &self,
        _context: &mut AgentContext,
        _tool_name: &str,
        _tool_args: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// 在 Agent 执行工具完成后调用
    async fn on_after_tool_execute(
        &self,
        _context: &mut AgentContext,
        _tool_name: &str,
        _result: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// 在 Agent 完成一轮 Reasoning-Action Loop 后调用
    async fn on_after_loop(
        &self,
        _context: &mut AgentContext,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}
