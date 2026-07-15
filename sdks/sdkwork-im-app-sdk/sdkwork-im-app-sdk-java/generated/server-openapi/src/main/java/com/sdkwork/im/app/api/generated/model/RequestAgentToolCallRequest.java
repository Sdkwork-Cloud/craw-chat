package com.sdkwork.im.app.api.generated.model;


public class RequestAgentToolCallRequest {
    private String executionId;
    private String toolCallId;
    private String toolName;
    private String argumentsPayload;

    public String getExecutionId() {
        return this.executionId;
    }

    public void setExecutionId(String executionId) {
        this.executionId = executionId;
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
}
