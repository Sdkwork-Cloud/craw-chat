package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class SdkCompatibilityBaselineResponse {
    private String appSdkFamily;
    private String backendSdkFamily;
    private String imSdkFamily;
    private String rtcSdkFamily;
    private List<String> matrixClientTypes;
    private String protocolGovernancePath;
    private String protocolRegistryPath;

    public String getAppSdkFamily() {
        return this.appSdkFamily;
    }

    public void setAppSdkFamily(String appSdkFamily) {
        this.appSdkFamily = appSdkFamily;
    }

    public String getBackendSdkFamily() {
        return this.backendSdkFamily;
    }

    public void setBackendSdkFamily(String backendSdkFamily) {
        this.backendSdkFamily = backendSdkFamily;
    }

    public String getImSdkFamily() {
        return this.imSdkFamily;
    }

    public void setImSdkFamily(String imSdkFamily) {
        this.imSdkFamily = imSdkFamily;
    }

    public String getRtcSdkFamily() {
        return this.rtcSdkFamily;
    }

    public void setRtcSdkFamily(String rtcSdkFamily) {
        this.rtcSdkFamily = rtcSdkFamily;
    }

    public List<String> getMatrixClientTypes() {
        return this.matrixClientTypes;
    }

    public void setMatrixClientTypes(List<String> matrixClientTypes) {
        this.matrixClientTypes = matrixClientTypes;
    }

    public String getProtocolGovernancePath() {
        return this.protocolGovernancePath;
    }

    public void setProtocolGovernancePath(String protocolGovernancePath) {
        this.protocolGovernancePath = protocolGovernancePath;
    }

    public String getProtocolRegistryPath() {
        return this.protocolRegistryPath;
    }

    public void setProtocolRegistryPath(String protocolRegistryPath) {
        this.protocolRegistryPath = protocolRegistryPath;
    }
}
