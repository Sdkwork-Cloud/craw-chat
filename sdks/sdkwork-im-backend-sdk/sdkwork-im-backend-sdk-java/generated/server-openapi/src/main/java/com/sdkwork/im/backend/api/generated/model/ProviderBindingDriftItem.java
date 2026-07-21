package com.sdkwork.im.backend.api.generated.model;


public class ProviderBindingDriftItem {
    private String tenantId;
    private String domain;
    private String baselineSelectedPluginId;
    private String selectedPluginId;
    private String baselineSelectionSource;
    private String selectionSource;
    private String driftKind;

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getDomain() {
        return this.domain;
    }

    public void setDomain(String domain) {
        this.domain = domain;
    }

    public String getBaselineSelectedPluginId() {
        return this.baselineSelectedPluginId;
    }

    public void setBaselineSelectedPluginId(String baselineSelectedPluginId) {
        this.baselineSelectedPluginId = baselineSelectedPluginId;
    }

    public String getSelectedPluginId() {
        return this.selectedPluginId;
    }

    public void setSelectedPluginId(String selectedPluginId) {
        this.selectedPluginId = selectedPluginId;
    }

    public String getBaselineSelectionSource() {
        return this.baselineSelectionSource;
    }

    public void setBaselineSelectionSource(String baselineSelectionSource) {
        this.baselineSelectionSource = baselineSelectionSource;
    }

    public String getSelectionSource() {
        return this.selectionSource;
    }

    public void setSelectionSource(String selectionSource) {
        this.selectionSource = selectionSource;
    }

    public String getDriftKind() {
        return this.driftKind;
    }

    public void setDriftKind(String driftKind) {
        this.driftKind = driftKind;
    }
}
