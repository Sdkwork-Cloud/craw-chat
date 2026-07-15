package com.sdkwork.im.app.api.generated.model;


public class ArchiveGroupConversationResponse {
    private Boolean accepted;
    private String resourceId;
    private String status;
    private String archiveEventId;
    private String archivedAt;
    private Boolean knowledgebaseArchiveScheduled;

    public Boolean getAccepted() {
        return this.accepted;
    }

    public void setAccepted(Boolean accepted) {
        this.accepted = accepted;
    }

    public String getResourceId() {
        return this.resourceId;
    }

    public void setResourceId(String resourceId) {
        this.resourceId = resourceId;
    }

    public String getStatus() {
        return this.status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public String getArchiveEventId() {
        return this.archiveEventId;
    }

    public void setArchiveEventId(String archiveEventId) {
        this.archiveEventId = archiveEventId;
    }

    public String getArchivedAt() {
        return this.archivedAt;
    }

    public void setArchivedAt(String archivedAt) {
        this.archivedAt = archivedAt;
    }

    public Boolean getKnowledgebaseArchiveScheduled() {
        return this.knowledgebaseArchiveScheduled;
    }

    public void setKnowledgebaseArchiveScheduled(Boolean knowledgebaseArchiveScheduled) {
        this.knowledgebaseArchiveScheduled = knowledgebaseArchiveScheduled;
    }
}
