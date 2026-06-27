package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class RealtimeSubscriptionSyncRequest {
    private String deviceId;
    private List<String> conversations;
    private List<RealtimeSubscriptionItemInput> items;

    public String getDeviceId() {
        return this.deviceId;
    }

    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public List<String> getConversations() {
        return this.conversations;
    }

    public void setConversations(List<String> conversations) {
        this.conversations = conversations;
    }

    public List<RealtimeSubscriptionItemInput> getItems() {
        return this.items;
    }

    public void setItems(List<RealtimeSubscriptionItemInput> items) {
        this.items = items;
    }
}
