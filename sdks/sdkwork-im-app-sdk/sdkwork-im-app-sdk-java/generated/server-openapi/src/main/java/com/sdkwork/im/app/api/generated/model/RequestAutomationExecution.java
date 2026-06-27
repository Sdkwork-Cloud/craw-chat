package com.sdkwork.im.app.api.generated.model;


public class RequestAutomationExecution {
    private String executionId;
    private String triggerType;
    private String targetKind;
    private String targetRef;
    private String inputPayload;

    public String getExecutionId() {
        return this.executionId;
    }
    
    public void setExecutionId(String executionId) {
        this.executionId = executionId;
    }

    public String getTriggerType() {
        return this.triggerType;
    }
    
    public void setTriggerType(String triggerType) {
        this.triggerType = triggerType;
    }

    public String getTargetKind() {
        return this.targetKind;
    }
    
    public void setTargetKind(String targetKind) {
        this.targetKind = targetKind;
    }

    public String getTargetRef() {
        return this.targetRef;
    }
    
    public void setTargetRef(String targetRef) {
        this.targetRef = targetRef;
    }

    public String getInputPayload() {
        return this.inputPayload;
    }
    
    public void setInputPayload(String inputPayload) {
        this.inputPayload = inputPayload;
    }
}
