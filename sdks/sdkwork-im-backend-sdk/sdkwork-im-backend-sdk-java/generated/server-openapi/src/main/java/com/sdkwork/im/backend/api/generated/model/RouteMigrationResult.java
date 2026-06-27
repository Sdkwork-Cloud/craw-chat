package com.sdkwork.im.backend.api.generated.model;


public class RouteMigrationResult {
    private Integer migratedRouteCount;
    private String sourceDrainStatus;
    private String sourceNodeId;
    private String sourceRebalanceState;
    private String targetDrainStatus;
    private String targetNodeId;
    private String targetRebalanceState;

    public Integer getMigratedRouteCount() {
        return this.migratedRouteCount;
    }
    
    public void setMigratedRouteCount(Integer migratedRouteCount) {
        this.migratedRouteCount = migratedRouteCount;
    }

    public String getSourceDrainStatus() {
        return this.sourceDrainStatus;
    }
    
    public void setSourceDrainStatus(String sourceDrainStatus) {
        this.sourceDrainStatus = sourceDrainStatus;
    }

    public String getSourceNodeId() {
        return this.sourceNodeId;
    }
    
    public void setSourceNodeId(String sourceNodeId) {
        this.sourceNodeId = sourceNodeId;
    }

    public String getSourceRebalanceState() {
        return this.sourceRebalanceState;
    }
    
    public void setSourceRebalanceState(String sourceRebalanceState) {
        this.sourceRebalanceState = sourceRebalanceState;
    }

    public String getTargetDrainStatus() {
        return this.targetDrainStatus;
    }
    
    public void setTargetDrainStatus(String targetDrainStatus) {
        this.targetDrainStatus = targetDrainStatus;
    }

    public String getTargetNodeId() {
        return this.targetNodeId;
    }
    
    public void setTargetNodeId(String targetNodeId) {
        this.targetNodeId = targetNodeId;
    }

    public String getTargetRebalanceState() {
        return this.targetRebalanceState;
    }
    
    public void setTargetRebalanceState(String targetRebalanceState) {
        this.targetRebalanceState = targetRebalanceState;
    }
}
