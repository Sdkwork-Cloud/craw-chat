package com.sdkwork.im.sdk.generated.model;


public class CreateAgentHandoffRequest {
    private String conversationId;
    private String targetId;
    private String targetKind;
    private String handoffSessionId;
    private String handoffReason;

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getTargetId() {
        return this.targetId;
    }

    public void setTargetId(String targetId) {
        this.targetId = targetId;
    }

    public String getTargetKind() {
        return this.targetKind;
    }

    public void setTargetKind(String targetKind) {
        this.targetKind = targetKind;
    }

    public String getHandoffSessionId() {
        return this.handoffSessionId;
    }

    public void setHandoffSessionId(String handoffSessionId) {
        this.handoffSessionId = handoffSessionId;
    }

    public String getHandoffReason() {
        return this.handoffReason;
    }

    public void setHandoffReason(String handoffReason) {
        this.handoffReason = handoffReason;
    }
}
