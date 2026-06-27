package com.sdkwork.im.backend.api.generated.model;

import java.util.List;

public class ProtocolSchemaResponse {
    private List<String> bindingProtocols;
    private String kind;
    private List<String> requiredCapabilities;
    private String schema;
    private String stage;
    private List<String> supportedConsumers;

    public List<String> getBindingProtocols() {
        return this.bindingProtocols;
    }
    
    public void setBindingProtocols(List<String> bindingProtocols) {
        this.bindingProtocols = bindingProtocols;
    }

    public String getKind() {
        return this.kind;
    }
    
    public void setKind(String kind) {
        this.kind = kind;
    }

    public List<String> getRequiredCapabilities() {
        return this.requiredCapabilities;
    }
    
    public void setRequiredCapabilities(List<String> requiredCapabilities) {
        this.requiredCapabilities = requiredCapabilities;
    }

    public String getSchema() {
        return this.schema;
    }
    
    public void setSchema(String schema) {
        this.schema = schema;
    }

    public String getStage() {
        return this.stage;
    }
    
    public void setStage(String stage) {
        this.stage = stage;
    }

    public List<String> getSupportedConsumers() {
        return this.supportedConsumers;
    }
    
    public void setSupportedConsumers(List<String> supportedConsumers) {
        this.supportedConsumers = supportedConsumers;
    }
}
