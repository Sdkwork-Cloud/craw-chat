package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class UpdateConversationAgentsRequest {
    private Integer expectedGeneration;
    private List<ConversationAgentAssignment> agentAssignments;

    public Integer getExpectedGeneration() {
        return this.expectedGeneration;
    }

    public void setExpectedGeneration(Integer expectedGeneration) {
        this.expectedGeneration = expectedGeneration;
    }

    public List<ConversationAgentAssignment> getAgentAssignments() {
        return this.agentAssignments;
    }

    public void setAgentAssignments(List<ConversationAgentAssignment> agentAssignments) {
        this.agentAssignments = agentAssignments;
    }
}
