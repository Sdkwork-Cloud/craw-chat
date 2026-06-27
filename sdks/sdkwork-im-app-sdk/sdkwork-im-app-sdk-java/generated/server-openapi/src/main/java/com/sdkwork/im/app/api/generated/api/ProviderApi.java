package com.sdkwork.im.app.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class ProviderApi {
    private final HttpClient client;
    
    public ProviderApi(HttpClient client) {
        this.client = client;
    }

    /** Retrieve media provider health */
    public Map<String, Object> mediaHealthRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/media/provider_health"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve principal-profile provider health */
    public Map<String, Object> principalProfileHealthRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/principal/profiles/provider_health"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }




}
