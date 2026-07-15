package com.sdkwork.im.app.api.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.api.AutomationApi;
import com.sdkwork.im.app.api.generated.api.NotificationsApi;
import com.sdkwork.im.app.api.generated.api.PortalApi;
import com.sdkwork.im.app.api.generated.api.ProviderApi;
import com.sdkwork.im.app.api.generated.api.ChatApi;

public class SdkworkImAppClient {
    private final HttpClient httpClient;
    private AutomationApi automation;
    private NotificationsApi notifications;
    private PortalApi portal;
    private ProviderApi provider;
    private ChatApi chat;

    public SdkworkImAppClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.automation = new AutomationApi(httpClient);
        this.notifications = new NotificationsApi(httpClient);
        this.portal = new PortalApi(httpClient);
        this.provider = new ProviderApi(httpClient);
        this.chat = new ChatApi(httpClient);
    }

    public SdkworkImAppClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.automation = new AutomationApi(httpClient);
        this.notifications = new NotificationsApi(httpClient);
        this.portal = new PortalApi(httpClient);
        this.provider = new ProviderApi(httpClient);
        this.chat = new ChatApi(httpClient);
    }

    public AutomationApi getAutomation() {
        return this.automation;
    }

    public NotificationsApi getNotifications() {
        return this.notifications;
    }

    public PortalApi getPortal() {
        return this.portal;
    }

    public ProviderApi getProvider() {
        return this.provider;
    }

    public ChatApi getChat() {
        return this.chat;
    }
    public SdkworkImAppClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkImAppClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkImAppClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
