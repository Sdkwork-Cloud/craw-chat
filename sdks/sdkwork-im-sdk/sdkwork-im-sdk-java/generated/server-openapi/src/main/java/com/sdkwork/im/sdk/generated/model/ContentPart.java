package com.sdkwork.im.sdk.generated.model;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.EXISTING_PROPERTY, property = "kind", visible = true)
@JsonSubTypes({
    @JsonSubTypes.Type(value = TextContentPart.class, name = "text"),
    @JsonSubTypes.Type(value = DataContentPart.class, name = "data"),
    @JsonSubTypes.Type(value = MediaContentPart.class, name = "media"),
    @JsonSubTypes.Type(value = SignalContentPart.class, name = "signal"),
    @JsonSubTypes.Type(value = StreamRefContentPart.class, name = "stream_ref")
})
public abstract class ContentPart {
}
