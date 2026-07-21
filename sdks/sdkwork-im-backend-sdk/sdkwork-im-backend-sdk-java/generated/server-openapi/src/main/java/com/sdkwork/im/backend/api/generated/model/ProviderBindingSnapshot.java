package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class ProviderBindingSnapshot {
    private String interfaceVersion;
    private String tenantId;
    private List<ProviderBindingItem> effectiveBindings;
    private List<String> precedence;

    public String getInterfaceVersion() {
        return this.interfaceVersion;
    }

    public void setInterfaceVersion(String interfaceVersion) {
        this.interfaceVersion = interfaceVersion;
    }

    public String getTenantId() {
        return this.tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public List<ProviderBindingItem> getEffectiveBindings() {
        return this.effectiveBindings;
    }

    public void setEffectiveBindings(List<ProviderBindingItem> effectiveBindings) {
        this.effectiveBindings = effectiveBindings;
    }

    public List<String> getPrecedence() {
        return this.precedence;
    }

    public void setPrecedence(List<String> precedence) {
        this.precedence = precedence;
    }
}
