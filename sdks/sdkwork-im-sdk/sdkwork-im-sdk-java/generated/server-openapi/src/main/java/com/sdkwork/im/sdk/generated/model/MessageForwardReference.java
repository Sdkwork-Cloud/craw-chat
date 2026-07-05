package com.sdkwork.im.sdk.generated.model;


public class MessageForwardReference {
    private String originalMessageId;
    private String originalConversationId;
    private String originalSenderId;
    private String originalSenderKind;
    private String originalSenderDisplayName;
    private String originalOccurredAt;
    private String forwardedAt;
    private String contentPreview;
    private Integer forwardCount;

    public String getOriginalMessageId() {
        return this.originalMessageId;
    }

    public void setOriginalMessageId(String originalMessageId) {
        this.originalMessageId = originalMessageId;
    }

    public String getOriginalConversationId() {
        return this.originalConversationId;
    }

    public void setOriginalConversationId(String originalConversationId) {
        this.originalConversationId = originalConversationId;
    }

    public String getOriginalSenderId() {
        return this.originalSenderId;
    }

    public void setOriginalSenderId(String originalSenderId) {
        this.originalSenderId = originalSenderId;
    }

    public String getOriginalSenderKind() {
        return this.originalSenderKind;
    }

    public void setOriginalSenderKind(String originalSenderKind) {
        this.originalSenderKind = originalSenderKind;
    }

    public String getOriginalSenderDisplayName() {
        return this.originalSenderDisplayName;
    }

    public void setOriginalSenderDisplayName(String originalSenderDisplayName) {
        this.originalSenderDisplayName = originalSenderDisplayName;
    }

    public String getOriginalOccurredAt() {
        return this.originalOccurredAt;
    }

    public void setOriginalOccurredAt(String originalOccurredAt) {
        this.originalOccurredAt = originalOccurredAt;
    }

    public String getForwardedAt() {
        return this.forwardedAt;
    }

    public void setForwardedAt(String forwardedAt) {
        this.forwardedAt = forwardedAt;
    }

    public String getContentPreview() {
        return this.contentPreview;
    }

    public void setContentPreview(String contentPreview) {
        this.contentPreview = contentPreview;
    }

    public Integer getForwardCount() {
        return this.forwardCount;
    }

    public void setForwardCount(Integer forwardCount) {
        this.forwardCount = forwardCount;
    }
}
