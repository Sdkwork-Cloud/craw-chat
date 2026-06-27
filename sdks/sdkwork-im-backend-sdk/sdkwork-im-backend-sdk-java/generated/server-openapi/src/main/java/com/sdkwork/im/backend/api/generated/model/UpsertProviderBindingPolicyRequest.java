package com.sdkwork.im.backend.api.generated.model;


public class UpsertProviderBindingPolicyRequest {
    private String domain;
    private Integer expectedBaseVersion;
    private String pluginId;
    private String tenantId;

    public String getDomain() {
        return this.domain;
    }
    
    public void setDomain(String domain) {
        this.domain = domain;
    }

    public Integer getExpectedBaseVersion() {
        return this.expectedBaseVersion;
    }
    
    public void setExpectedBaseVersion(Integer expectedBaseVersion) {
        this.expectedBaseVersion = expectedBaseVersion;
    }

    public String getPluginId() {
        return this.pluginId;
    }
    
    public void setPluginId(String pluginId) {
        this.pluginId = pluginId;
    }

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }
}
