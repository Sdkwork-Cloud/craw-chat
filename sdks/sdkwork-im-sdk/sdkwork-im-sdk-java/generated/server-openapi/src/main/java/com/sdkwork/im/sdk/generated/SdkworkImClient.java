package com.sdkwork.im.sdk.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.api.PresenceApi;
import com.sdkwork.im.sdk.generated.api.RealtimeApi;
import com.sdkwork.im.sdk.generated.api.CallsApi;
import com.sdkwork.im.sdk.generated.api.SocialApi;
import com.sdkwork.im.sdk.generated.api.ChatApi;
import com.sdkwork.im.sdk.generated.api.StreamsApi;
import com.sdkwork.im.sdk.generated.api.SpacesApi;

public class SdkworkImClient {
    private final HttpClient httpClient;
    private PresenceApi presence;
    private RealtimeApi realtime;
    private CallsApi calls;
    private SocialApi social;
    private ChatApi chat;
    private StreamsApi streams;
    private SpacesApi spaces;

    public SdkworkImClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.calls = new CallsApi(httpClient);
        this.social = new SocialApi(httpClient);
        this.chat = new ChatApi(httpClient);
        this.streams = new StreamsApi(httpClient);
        this.spaces = new SpacesApi(httpClient);
    }

    public SdkworkImClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.calls = new CallsApi(httpClient);
        this.social = new SocialApi(httpClient);
        this.chat = new ChatApi(httpClient);
        this.streams = new StreamsApi(httpClient);
        this.spaces = new SpacesApi(httpClient);
    }

    public PresenceApi getPresence() {
        return this.presence;
    }

    public RealtimeApi getRealtime() {
        return this.realtime;
    }

    public CallsApi getCalls() {
        return this.calls;
    }

    public SocialApi getSocial() {
        return this.social;
    }

    public ChatApi getChat() {
        return this.chat;
    }

    public StreamsApi getStreams() {
        return this.streams;
    }

    public SpacesApi getSpaces() {
        return this.spaces;
    }
    public SdkworkImClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkImClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkImClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
