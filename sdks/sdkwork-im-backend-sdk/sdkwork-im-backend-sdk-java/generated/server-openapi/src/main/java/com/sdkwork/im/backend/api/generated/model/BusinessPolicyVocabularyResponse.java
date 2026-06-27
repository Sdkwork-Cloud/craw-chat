package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class BusinessPolicyVocabularyResponse {
    private String capabilityFlagsField;
    private String historyVisibilityField;
    private List<String> historyVisibilityModes;
    private String policyVersionField;
    private String retentionPolicyRefField;
    private List<String> retentionPolicyScopes;

    public String getCapabilityFlagsField() {
        return this.capabilityFlagsField;
    }
    
    public void setCapabilityFlagsField(String capabilityFlagsField) {
        this.capabilityFlagsField = capabilityFlagsField;
    }

    public String getHistoryVisibilityField() {
        return this.historyVisibilityField;
    }
    
    public void setHistoryVisibilityField(String historyVisibilityField) {
        this.historyVisibilityField = historyVisibilityField;
    }

    public List<String> getHistoryVisibilityModes() {
        return this.historyVisibilityModes;
    }
    
    public void setHistoryVisibilityModes(List<String> historyVisibilityModes) {
        this.historyVisibilityModes = historyVisibilityModes;
    }

    public String getPolicyVersionField() {
        return this.policyVersionField;
    }
    
    public void setPolicyVersionField(String policyVersionField) {
        this.policyVersionField = policyVersionField;
    }

    public String getRetentionPolicyRefField() {
        return this.retentionPolicyRefField;
    }
    
    public void setRetentionPolicyRefField(String retentionPolicyRefField) {
        this.retentionPolicyRefField = retentionPolicyRefField;
    }

    public List<String> getRetentionPolicyScopes() {
        return this.retentionPolicyScopes;
    }
    
    public void setRetentionPolicyScopes(List<String> retentionPolicyScopes) {
        this.retentionPolicyScopes = retentionPolicyScopes;
    }
}
