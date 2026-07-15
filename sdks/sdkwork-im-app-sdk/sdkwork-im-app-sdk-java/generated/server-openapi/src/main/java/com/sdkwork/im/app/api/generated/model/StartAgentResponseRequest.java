package com.sdkwork.im.app.api.generated.model;


public class StartAgentResponseRequest {
    private String executionId;
    private String streamId;
    private String streamType;
    private String conversationId;
    private String schemaRef;
    private String memberId;
    private AgentSubject agent;

    public String getExecutionId() {
        return this.executionId;
    }

    public void setExecutionId(String executionId) {
        this.executionId = executionId;
    }

    public String getStreamId() {
        return this.streamId;
    }

    public void setStreamId(String streamId) {
        this.streamId = streamId;
    }

    public String getStreamType() {
        return this.streamType;
    }

    public void setStreamType(String streamType) {
        this.streamType = streamType;
    }

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }

    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getMemberId() {
        return this.memberId;
    }

    public void setMemberId(String memberId) {
        this.memberId = memberId;
    }

    public AgentSubject getAgent() {
        return this.agent;
    }

    public void setAgent(AgentSubject agent) {
        this.agent = agent;
    }
}
