pub const PREFIX: &str = "/app/v3/api/automation";

pub const EXECUTIONS: &str = "/app/v3/api/automation/executions";
pub const AGENT_RESPONSES: &str = "/app/v3/api/automation/agent_responses";
pub const AGENT_RESPONSE_FRAMES: &str =
    "/app/v3/api/automation/agent_responses/{streamId}/frames";
pub const AGENT_RESPONSE_COMPLETE: &str =
    "/app/v3/api/automation/agent_responses/{streamId}/complete";
pub const AGENT_TOOL_CALLS: &str = "/app/v3/api/automation/agent_tool_calls";
pub const EXECUTION_TOOL_CALL_COMPLETE: &str =
    "/app/v3/api/automation/executions/{executionId}/agent_tool_calls/{toolCallId}/complete";
pub const EXECUTION: &str = "/app/v3/api/automation/executions/{executionId}";
pub const GOVERNANCE: &str = "/backend/v3/api/automation/governance";
