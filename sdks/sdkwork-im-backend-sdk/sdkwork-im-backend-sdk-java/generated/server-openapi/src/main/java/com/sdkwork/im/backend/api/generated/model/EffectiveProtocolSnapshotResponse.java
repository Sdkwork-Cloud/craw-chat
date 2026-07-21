package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class EffectiveProtocolSnapshotResponse {
    private List<String> allowedBindings;
    private List<String> allowedCodecs;
    private List<String> enabledCapabilities;
    private Boolean killSwitchActive;
    private List<String> precedence;
    private String protocolVersion;
    private String quotaProfileId;
    private String releaseChannel;

    public List<String> getAllowedBindings() {
        return this.allowedBindings;
    }

    public void setAllowedBindings(List<String> allowedBindings) {
        this.allowedBindings = allowedBindings;
    }

    public List<String> getAllowedCodecs() {
        return this.allowedCodecs;
    }

    public void setAllowedCodecs(List<String> allowedCodecs) {
        this.allowedCodecs = allowedCodecs;
    }

    public List<String> getEnabledCapabilities() {
        return this.enabledCapabilities;
    }

    public void setEnabledCapabilities(List<String> enabledCapabilities) {
        this.enabledCapabilities = enabledCapabilities;
    }

    public Boolean getKillSwitchActive() {
        return this.killSwitchActive;
    }

    public void setKillSwitchActive(Boolean killSwitchActive) {
        this.killSwitchActive = killSwitchActive;
    }

    public List<String> getPrecedence() {
        return this.precedence;
    }

    public void setPrecedence(List<String> precedence) {
        this.precedence = precedence;
    }

    public String getProtocolVersion() {
        return this.protocolVersion;
    }

    public void setProtocolVersion(String protocolVersion) {
        this.protocolVersion = protocolVersion;
    }

    public String getQuotaProfileId() {
        return this.quotaProfileId;
    }

    public void setQuotaProfileId(String quotaProfileId) {
        this.quotaProfileId = quotaProfileId;
    }

    public String getReleaseChannel() {
        return this.releaseChannel;
    }

    public void setReleaseChannel(String releaseChannel) {
        this.releaseChannel = releaseChannel;
    }
}
