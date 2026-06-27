package com.sdkwork.im.sdk.generated.model;


public class PostedMessageResponse {
    private String conversationId;
    private String messageId;
    private Integer messageSeq;
    private MessageBody body;
    private String occurredAt;

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getMessageId() {
        return this.messageId;
    }

    public void setMessageId(String messageId) {
        this.messageId = messageId;
    }

    public Integer getMessageSeq() {
        return this.messageSeq;
    }

    public void setMessageSeq(Integer messageSeq) {
        this.messageSeq = messageSeq;
    }

    public MessageBody getBody() {
        return this.body;
    }

    public void setBody(MessageBody body) {
        this.body = body;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }

    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }
}
