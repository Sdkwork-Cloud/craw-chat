package com.sdkwork.im.backend.api.generated.model;


public class RouteNodeLifecycle {
    private String drainStatus;
    private String nodeId;
    private Integer ownedRouteCount;
    private String rebalanceState;

    public String getDrainStatus() {
        return this.drainStatus;
    }
    
    public void setDrainStatus(String drainStatus) {
        this.drainStatus = drainStatus;
    }

    public String getNodeId() {
        return this.nodeId;
    }
    
    public void setNodeId(String nodeId) {
        this.nodeId = nodeId;
    }

    public Integer getOwnedRouteCount() {
        return this.ownedRouteCount;
    }
    
    public void setOwnedRouteCount(Integer ownedRouteCount) {
        this.ownedRouteCount = ownedRouteCount;
    }

    public String getRebalanceState() {
        return this.rebalanceState;
    }
    
    public void setRebalanceState(String rebalanceState) {
        this.rebalanceState = rebalanceState;
    }
}
