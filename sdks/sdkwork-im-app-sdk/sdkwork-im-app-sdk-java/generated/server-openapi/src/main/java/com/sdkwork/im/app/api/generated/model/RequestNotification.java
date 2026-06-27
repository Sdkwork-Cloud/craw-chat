package com.sdkwork.im.app.api.generated.model;


public class RequestNotification {
    private String notificationId;
    private String sourceEventId;
    private String sourceEventType;
    private String category;
    private String channel;
    private String recipientId;
    private String recipientKind;
    private String title;
    private String body;
    private String payload;

    public String getNotificationId() {
        return this.notificationId;
    }
    
    public void setNotificationId(String notificationId) {
        this.notificationId = notificationId;
    }

    public String getSourceEventId() {
        return this.sourceEventId;
    }
    
    public void setSourceEventId(String sourceEventId) {
        this.sourceEventId = sourceEventId;
    }

    public String getSourceEventType() {
        return this.sourceEventType;
    }
    
    public void setSourceEventType(String sourceEventType) {
        this.sourceEventType = sourceEventType;
    }

    public String getCategory() {
        return this.category;
    }
    
    public void setCategory(String category) {
        this.category = category;
    }

    public String getChannel() {
        return this.channel;
    }
    
    public void setChannel(String channel) {
        this.channel = channel;
    }

    public String getRecipientId() {
        return this.recipientId;
    }
    
    public void setRecipientId(String recipientId) {
        this.recipientId = recipientId;
    }

    public String getRecipientKind() {
        return this.recipientKind;
    }
    
    public void setRecipientKind(String recipientKind) {
        this.recipientKind = recipientKind;
    }

    public String getTitle() {
        return this.title;
    }
    
    public void setTitle(String title) {
        this.title = title;
    }

    public String getBody() {
        return this.body;
    }
    
    public void setBody(String body) {
        this.body = body;
    }

    public String getPayload() {
        return this.payload;
    }
    
    public void setPayload(String payload) {
        this.payload = payload;
    }
}
