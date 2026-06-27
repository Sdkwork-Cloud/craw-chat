package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class RolloutPolicyResponse {
    private String cellSelector;
    private Boolean operatorOverride;
    private String policyId;
    private String regionSelector;
    private String releaseChannel;
    private List<String> tenantAllowlist;
    private Integer trafficPercent;

    public String getCellSelector() {
        return this.cellSelector;
    }
    
    public void setCellSelector(String cellSelector) {
        this.cellSelector = cellSelector;
    }

    public Boolean getOperatorOverride() {
        return this.operatorOverride;
    }
    
    public void setOperatorOverride(Boolean operatorOverride) {
        this.operatorOverride = operatorOverride;
    }

    public String getPolicyId() {
        return this.policyId;
    }
    
    public void setPolicyId(String policyId) {
        this.policyId = policyId;
    }

    public String getRegionSelector() {
        return this.regionSelector;
    }
    
    public void setRegionSelector(String regionSelector) {
        this.regionSelector = regionSelector;
    }

    public String getReleaseChannel() {
        return this.releaseChannel;
    }
    
    public void setReleaseChannel(String releaseChannel) {
        this.releaseChannel = releaseChannel;
    }

    public List<String> getTenantAllowlist() {
        return this.tenantAllowlist;
    }
    
    public void setTenantAllowlist(List<String> tenantAllowlist) {
        this.tenantAllowlist = tenantAllowlist;
    }

    public Integer getTrafficPercent() {
        return this.trafficPercent;
    }
    
    public void setTrafficPercent(Integer trafficPercent) {
        this.trafficPercent = trafficPercent;
    }
}
