package com.sdkwork.im.backend.api.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.api.OpsApi;
import com.sdkwork.im.backend.api.generated.api.AuditApi;
import com.sdkwork.im.backend.api.generated.api.AutomationApi;
import com.sdkwork.im.backend.api.generated.api.ControlApi;
import com.sdkwork.im.backend.api.generated.api.AdminApi;

public class SdkworkImBackendClient {
    private final HttpClient httpClient;
    private OpsApi ops;
    private AuditApi audit;
    private AutomationApi automation;
    private ControlApi control;
    private AdminApi admin;

    public SdkworkImBackendClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.ops = new OpsApi(httpClient);
        this.audit = new AuditApi(httpClient);
        this.automation = new AutomationApi(httpClient);
        this.control = new ControlApi(httpClient);
        this.admin = new AdminApi(httpClient);
    }

    public SdkworkImBackendClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.ops = new OpsApi(httpClient);
        this.audit = new AuditApi(httpClient);
        this.automation = new AutomationApi(httpClient);
        this.control = new ControlApi(httpClient);
        this.admin = new AdminApi(httpClient);
    }

    public OpsApi getOps() {
        return this.ops;
    }

    public AuditApi getAudit() {
        return this.audit;
    }

    public AutomationApi getAutomation() {
        return this.automation;
    }

    public ControlApi getControl() {
        return this.control;
    }

    public AdminApi getAdmin() {
        return this.admin;
    }
    public SdkworkImBackendClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkImBackendClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkImBackendClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
