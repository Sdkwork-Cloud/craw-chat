package com.sdkwork.im.backend.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class OpsApi {
    private final HttpClient client;
    
    public OpsApi(HttpClient client) {
        this.client = client;
    }

    /** Retrieve ops health */
    public Map<String, Object> healthRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/health"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve cluster state */
    public Map<String, Object> clusterRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/cluster"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve projection lag */
    public Map<String, Object> lagRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/lag"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve replay status */
    public Map<String, Object> replayStatusRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/replay_status"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve commercial readiness */
    public Map<String, Object> commercialReadinessRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/commercial_readiness"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Inspect runtime directory */
    public Map<String, Object> runtimeDirRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/runtime_dir"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** List provider bindings */
    public Map<String, Object> providerBindingsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/provider_bindings"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve provider binding drift */
    public Map<String, Object> providerBindingsDriftRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/provider_bindings/drift"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve diagnostics */
    public Map<String, Object> diagnosticsRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/ops/diagnostics"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }




}
