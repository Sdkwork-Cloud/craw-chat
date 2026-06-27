package com.sdkwork.im.sdk.generated.model;


public class SpaceChannelAccessRuleCreateRequest {
    private String ruleType;
    private String principalKind;
    private String principalId;
    private String permission;

    public String getRuleType() {
        return this.ruleType;
    }

    public void setRuleType(String ruleType) {
        this.ruleType = ruleType;
    }

    public String getPrincipalKind() {
        return this.principalKind;
    }

    public void setPrincipalKind(String principalKind) {
        this.principalKind = principalKind;
    }

    public String getPrincipalId() {
        return this.principalId;
    }

    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public String getPermission() {
        return this.permission;
    }

    public void setPermission(String permission) {
        this.permission = permission;
    }
}
