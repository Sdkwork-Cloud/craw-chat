package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class ClientCompatibilityResponse {
    private List<String> blockedExperimentalCapabilities;
    private String clientType;
    private String minimumProtocolVersion;
    private List<String> supportedBindings;
    private List<String> supportedCapabilities;
    private List<String> supportedCodecs;

    public List<String> getBlockedExperimentalCapabilities() {
        return this.blockedExperimentalCapabilities;
    }
    
    public void setBlockedExperimentalCapabilities(List<String> blockedExperimentalCapabilities) {
        this.blockedExperimentalCapabilities = blockedExperimentalCapabilities;
    }

    public String getClientType() {
        return this.clientType;
    }
    
    public void setClientType(String clientType) {
        this.clientType = clientType;
    }

    public String getMinimumProtocolVersion() {
        return this.minimumProtocolVersion;
    }
    
    public void setMinimumProtocolVersion(String minimumProtocolVersion) {
        this.minimumProtocolVersion = minimumProtocolVersion;
    }

    public List<String> getSupportedBindings() {
        return this.supportedBindings;
    }
    
    public void setSupportedBindings(List<String> supportedBindings) {
        this.supportedBindings = supportedBindings;
    }

    public List<String> getSupportedCapabilities() {
        return this.supportedCapabilities;
    }
    
    public void setSupportedCapabilities(List<String> supportedCapabilities) {
        this.supportedCapabilities = supportedCapabilities;
    }

    public List<String> getSupportedCodecs() {
        return this.supportedCodecs;
    }
    
    public void setSupportedCodecs(List<String> supportedCodecs) {
        this.supportedCodecs = supportedCodecs;
    }
}
