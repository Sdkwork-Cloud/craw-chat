package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class ConversationAgentAssignments {
    private Integer generation;
    private String source;
    private List<ConversationAgentAssignment> agents;

    public Integer getGeneration() {
        return this.generation;
    }

    public void setGeneration(Integer generation) {
        this.generation = generation;
    }

    public String getSource() {
        return this.source;
    }

    public void setSource(String source) {
        this.source = source;
    }

    public List<ConversationAgentAssignment> getAgents() {
        return this.agents;
    }

    public void setAgents(List<ConversationAgentAssignment> agents) {
        this.agents = agents;
    }
}
