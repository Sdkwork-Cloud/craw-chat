package com.sdkwork.im.app.api.generated.model;


public class AgentToolCall {
    private String tenantId;
    private String executionId;
    private String agentId;
    private String toolCallId;
    private String toolName;
    private String argumentsPayload;
    private String resultPayload;
    private String state;
    private String requestedAt;
    private String completedAt;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getExecutionId() {
        return this.executionId;
    }
    
    public void setExecutionId(String executionId) {
        this.executionId = executionId;
    }

    public String getAgentId() {
        return this.agentId;
    }
    
    public void setAgentId(String agentId) {
        this.agentId = agentId;
    }

    public String getToolCallId() {
        return this.toolCallId;
    }
    
    public void setToolCallId(String toolCallId) {
        this.toolCallId = toolCallId;
    }

    public String getToolName() {
        return this.toolName;
    }
    
    public void setToolName(String toolName) {
        this.toolName = toolName;
    }

    public String getArgumentsPayload() {
        return this.argumentsPayload;
    }
    
    public void setArgumentsPayload(String argumentsPayload) {
        this.argumentsPayload = argumentsPayload;
    }

    public String getResultPayload() {
        return this.resultPayload;
    }
    
    public void setResultPayload(String resultPayload) {
        this.resultPayload = resultPayload;
    }

    public String getState() {
        return this.state;
    }
    
    public void setState(String state) {
        this.state = state;
    }

    public String getRequestedAt() {
        return this.requestedAt;
    }
    
    public void setRequestedAt(String requestedAt) {
        this.requestedAt = requestedAt;
    }

    public String getCompletedAt() {
        return this.completedAt;
    }
    
    public void setCompletedAt(String completedAt) {
        this.completedAt = completedAt;
    }
}
