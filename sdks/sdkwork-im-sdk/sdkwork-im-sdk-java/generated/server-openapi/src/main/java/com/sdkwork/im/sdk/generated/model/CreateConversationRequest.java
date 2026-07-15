package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class CreateConversationRequest {
    private String conversationId;
    private String conversationType;
    private String groupName;
    private String clientRequestKey;
    private Boolean initializeKnowledgebase;
    private List<String> memberUserIds;
    private List<ConversationAgentAssignment> agentAssignments;
    private String policyVersion;
    private List<String> capabilityFlags;
    private String historyVisibility;
    private String retentionPolicyRef;

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getConversationType() {
        return this.conversationType;
    }

    public void setConversationType(String conversationType) {
        this.conversationType = conversationType;
    }

    public String getGroupName() {
        return this.groupName;
    }

    public void setGroupName(String groupName) {
        this.groupName = groupName;
    }

    public String getClientRequestKey() {
        return this.clientRequestKey;
    }

    public void setClientRequestKey(String clientRequestKey) {
        this.clientRequestKey = clientRequestKey;
    }

    public Boolean getInitializeKnowledgebase() {
        return this.initializeKnowledgebase;
    }

    public void setInitializeKnowledgebase(Boolean initializeKnowledgebase) {
        this.initializeKnowledgebase = initializeKnowledgebase;
    }

    public List<String> getMemberUserIds() {
        return this.memberUserIds;
    }

    public void setMemberUserIds(List<String> memberUserIds) {
        this.memberUserIds = memberUserIds;
    }

    public List<ConversationAgentAssignment> getAgentAssignments() {
        return this.agentAssignments;
    }

    public void setAgentAssignments(List<ConversationAgentAssignment> agentAssignments) {
        this.agentAssignments = agentAssignments;
    }

    public String getPolicyVersion() {
        return this.policyVersion;
    }

    public void setPolicyVersion(String policyVersion) {
        this.policyVersion = policyVersion;
    }

    public List<String> getCapabilityFlags() {
        return this.capabilityFlags;
    }

    public void setCapabilityFlags(List<String> capabilityFlags) {
        this.capabilityFlags = capabilityFlags;
    }

    public String getHistoryVisibility() {
        return this.historyVisibility;
    }

    public void setHistoryVisibility(String historyVisibility) {
        this.historyVisibility = historyVisibility;
    }

    public String getRetentionPolicyRef() {
        return this.retentionPolicyRef;
    }

    public void setRetentionPolicyRef(String retentionPolicyRef) {
        this.retentionPolicyRef = retentionPolicyRef;
    }
}
