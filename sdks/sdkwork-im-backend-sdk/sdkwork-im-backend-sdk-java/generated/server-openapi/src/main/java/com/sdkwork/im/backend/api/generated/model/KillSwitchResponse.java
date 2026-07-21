package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class KillSwitchResponse {
    private Boolean active;
    private List<String> disabledBindings;
    private List<String> disabledCapabilities;
    private List<String> disabledCodecs;
    private String reason;
    private String ruleId;

    public Boolean getActive() {
        return this.active;
    }

    public void setActive(Boolean active) {
        this.active = active;
    }

    public List<String> getDisabledBindings() {
        return this.disabledBindings;
    }

    public void setDisabledBindings(List<String> disabledBindings) {
        this.disabledBindings = disabledBindings;
    }

    public List<String> getDisabledCapabilities() {
        return this.disabledCapabilities;
    }

    public void setDisabledCapabilities(List<String> disabledCapabilities) {
        this.disabledCapabilities = disabledCapabilities;
    }

    public List<String> getDisabledCodecs() {
        return this.disabledCodecs;
    }

    public void setDisabledCodecs(List<String> disabledCodecs) {
        this.disabledCodecs = disabledCodecs;
    }

    public String getReason() {
        return this.reason;
    }

    public void setReason(String reason) {
        this.reason = reason;
    }

    public String getRuleId() {
        return this.ruleId;
    }

    public void setRuleId(String ruleId) {
        this.ruleId = ruleId;
    }
}
