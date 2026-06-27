package com.sdkwork.im.app.api.generated.model;


public class AutomationExecution {
    private String tenantId;
    private String principalId;
    private String principalKind;
    private String executionId;
    private String triggerType;
    private String targetKind;
    private String targetRef;
    private String inputPayload;
    private String outputPayload;
    private String state;
    private Integer retryCount;
    private String requestedAt;
    private String completedAt;
    private String failureReason;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getPrincipalId() {
        return this.principalId;
    }
    
    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public String getPrincipalKind() {
        return this.principalKind;
    }
    
    public void setPrincipalKind(String principalKind) {
        this.principalKind = principalKind;
    }

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

    public String getOutputPayload() {
        return this.outputPayload;
    }
    
    public void setOutputPayload(String outputPayload) {
        this.outputPayload = outputPayload;
    }

    public String getState() {
        return this.state;
    }
    
    public void setState(String state) {
        this.state = state;
    }

    public Integer getRetryCount() {
        return this.retryCount;
    }
    
    public void setRetryCount(Integer retryCount) {
        this.retryCount = retryCount;
    }

    public String getRequestedAt() {
        return this.requestedAt;
    }
    
    public void setRequestedAt(String requestedAt) {
        this.requestedAt = requestedAt;
    }

    public String getCompletedAt() {
        return this.completedAt;
    }
    
    public void setCompletedAt(String completedAt) {
        this.completedAt = completedAt;
    }

    public String getFailureReason() {
        return this.failureReason;
    }
    
    public void setFailureReason(String failureReason) {
        this.failureReason = failureReason;
    }
}
