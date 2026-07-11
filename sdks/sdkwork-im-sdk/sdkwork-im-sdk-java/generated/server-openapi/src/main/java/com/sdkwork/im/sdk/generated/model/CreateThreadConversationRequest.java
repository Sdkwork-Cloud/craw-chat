package com.sdkwork.im.sdk.generated.model;


public class CreateThreadConversationRequest {
    private String conversationId;
    private String parentConversationId;
    private String rootMessageId;

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getParentConversationId() {
        return this.parentConversationId;
    }

    public void setParentConversationId(String parentConversationId) {
        this.parentConversationId = parentConversationId;
    }

    public String getRootMessageId() {
        return this.rootMessageId;
    }

    public void setRootMessageId(String rootMessageId) {
        this.rootMessageId = rootMessageId;
    }
}
