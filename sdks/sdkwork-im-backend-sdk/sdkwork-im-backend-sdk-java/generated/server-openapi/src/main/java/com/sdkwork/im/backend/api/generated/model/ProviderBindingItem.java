package com.sdkwork.im.backend.api.generated.model;


public class ProviderBindingItem {
    private String domain;
    private String defaultPluginId;
    private String selectedPluginId;
    private String selectionSource;
    private Boolean tenantOverrideAllowed;

    public String getDomain() {
        return this.domain;
    }

    public void setDomain(String domain) {
        this.domain = domain;
    }

    public String getDefaultPluginId() {
        return this.defaultPluginId;
    }

    public void setDefaultPluginId(String defaultPluginId) {
        this.defaultPluginId = defaultPluginId;
    }

    public String getSelectedPluginId() {
        return this.selectedPluginId;
    }

    public void setSelectedPluginId(String selectedPluginId) {
        this.selectedPluginId = selectedPluginId;
    }

    public String getSelectionSource() {
        return this.selectionSource;
    }

    public void setSelectionSource(String selectionSource) {
        this.selectionSource = selectionSource;
    }

    public Boolean getTenantOverrideAllowed() {
        return this.tenantOverrideAllowed;
    }

    public void setTenantOverrideAllowed(Boolean tenantOverrideAllowed) {
        this.tenantOverrideAllowed = tenantOverrideAllowed;
    }
}
