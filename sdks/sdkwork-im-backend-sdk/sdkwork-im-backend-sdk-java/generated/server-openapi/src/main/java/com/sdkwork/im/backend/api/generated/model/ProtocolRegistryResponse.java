package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class ProtocolRegistryResponse {
    private List<String> bindings;
    private List<String> codecs;
    private List<ClientCompatibilityResponse> compatibilityMatrix;
    private String protocolVersion;
    private List<ProtocolSchemaResponse> schemas;

    public List<String> getBindings() {
        return this.bindings;
    }

    public void setBindings(List<String> bindings) {
        this.bindings = bindings;
    }

    public List<String> getCodecs() {
        return this.codecs;
    }

    public void setCodecs(List<String> codecs) {
        this.codecs = codecs;
    }

    public List<ClientCompatibilityResponse> getCompatibilityMatrix() {
        return this.compatibilityMatrix;
    }

    public void setCompatibilityMatrix(List<ClientCompatibilityResponse> compatibilityMatrix) {
        this.compatibilityMatrix = compatibilityMatrix;
    }

    public String getProtocolVersion() {
        return this.protocolVersion;
    }

    public void setProtocolVersion(String protocolVersion) {
        this.protocolVersion = protocolVersion;
    }

    public List<ProtocolSchemaResponse> getSchemas() {
        return this.schemas;
    }

    public void setSchemas(List<ProtocolSchemaResponse> schemas) {
        this.schemas = schemas;
    }
}
