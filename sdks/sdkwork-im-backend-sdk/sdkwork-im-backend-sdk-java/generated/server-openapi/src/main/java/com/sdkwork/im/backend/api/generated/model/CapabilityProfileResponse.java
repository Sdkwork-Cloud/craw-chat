package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class CapabilityProfileResponse {
    private List<String> enabledCapabilities;
    private List<String> experimentalCapabilities;
    private String profileId;
    private String releaseChannel;

    public List<String> getEnabledCapabilities() {
        return this.enabledCapabilities;
    }
    
    public void setEnabledCapabilities(List<String> enabledCapabilities) {
        this.enabledCapabilities = enabledCapabilities;
    }

    public List<String> getExperimentalCapabilities() {
        return this.experimentalCapabilities;
    }
    
    public void setExperimentalCapabilities(List<String> experimentalCapabilities) {
        this.experimentalCapabilities = experimentalCapabilities;
    }

    public String getProfileId() {
        return this.profileId;
    }
    
    public void setProfileId(String profileId) {
        this.profileId = profileId;
    }

    public String getReleaseChannel() {
        return this.releaseChannel;
    }
    
    public void setReleaseChannel(String releaseChannel) {
        this.releaseChannel = releaseChannel;
    }
}
