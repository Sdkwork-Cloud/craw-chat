package com.sdkwork.im.sdk.generated.model;


public class ConversationInboxEntry {
    private String tenantId;
    private String conversationId;
    private Boolean agentHandoff;
    private String conversationType;
    private String displayName;
    private String avatarUrl;
    private String displaySource;
    private ConversationInboxPeerView peer;
    private ConversationInboxPreferencesView preferences;
    private String lastActivityAt;
    private String lastMessageId;
    private String lastSenderId;
    private Integer messageCount;
    private Integer lastMessageSeq;
    private String lastSummary;
    private String lastMessageAt;
    private Integer unreadCount;

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public Boolean getAgentHandoff() {
        return this.agentHandoff;
    }

    public void setAgentHandoff(Boolean agentHandoff) {
        this.agentHandoff = agentHandoff;
    }

    public String getConversationType() {
        return this.conversationType;
    }

    public void setConversationType(String conversationType) {
        this.conversationType = conversationType;
    }

    public String getDisplayName() {
        return this.displayName;
    }

    public void setDisplayName(String displayName) {
        this.displayName = displayName;
    }

    public String getAvatarUrl() {
        return this.avatarUrl;
    }

    public void setAvatarUrl(String avatarUrl) {
        this.avatarUrl = avatarUrl;
    }

    public String getDisplaySource() {
        return this.displaySource;
    }

    public void setDisplaySource(String displaySource) {
        this.displaySource = displaySource;
    }

    public ConversationInboxPeerView getPeer() {
        return this.peer;
    }

    public void setPeer(ConversationInboxPeerView peer) {
        this.peer = peer;
    }

    public ConversationInboxPreferencesView getPreferences() {
        return this.preferences;
    }

    public void setPreferences(ConversationInboxPreferencesView preferences) {
        this.preferences = preferences;
    }

    public String getLastActivityAt() {
        return this.lastActivityAt;
    }

    public void setLastActivityAt(String lastActivityAt) {
        this.lastActivityAt = lastActivityAt;
    }

    public String getLastMessageId() {
        return this.lastMessageId;
    }

    public void setLastMessageId(String lastMessageId) {
        this.lastMessageId = lastMessageId;
    }

    public String getLastSenderId() {
        return this.lastSenderId;
    }

    public void setLastSenderId(String lastSenderId) {
        this.lastSenderId = lastSenderId;
    }

    public Integer getMessageCount() {
        return this.messageCount;
    }

    public void setMessageCount(Integer messageCount) {
        this.messageCount = messageCount;
    }

    public Integer getLastMessageSeq() {
        return this.lastMessageSeq;
    }

    public void setLastMessageSeq(Integer lastMessageSeq) {
        this.lastMessageSeq = lastMessageSeq;
    }

    public String getLastSummary() {
        return this.lastSummary;
    }

    public void setLastSummary(String lastSummary) {
        this.lastSummary = lastSummary;
    }

    public String getLastMessageAt() {
        return this.lastMessageAt;
    }

    public void setLastMessageAt(String lastMessageAt) {
        this.lastMessageAt = lastMessageAt;
    }

    public Integer getUnreadCount() {
        return this.unreadCount;
    }

    public void setUnreadCount(Integer unreadCount) {
        this.unreadCount = unreadCount;
    }
}
