package com.sdkwork.im.sdk.generated.model;


public class MentionContentPart extends ContentPart {
    private String kind;
    private String targetKind;
    private String targetId;
    private String displayText;
    private Integer assignmentGeneration;

    public String getKind() {
        return this.kind;
    }

    public void setKind(String kind) {
        this.kind = kind;
    }

    public String getTargetKind() {
        return this.targetKind;
    }

    public void setTargetKind(String targetKind) {
        this.targetKind = targetKind;
    }

    public String getTargetId() {
        return this.targetId;
    }

    public void setTargetId(String targetId) {
        this.targetId = targetId;
    }

    public String getDisplayText() {
        return this.displayText;
    }

    public void setDisplayText(String displayText) {
        this.displayText = displayText;
    }

    public Integer getAssignmentGeneration() {
        return this.assignmentGeneration;
    }

    public void setAssignmentGeneration(Integer assignmentGeneration) {
        this.assignmentGeneration = assignmentGeneration;
    }
}
