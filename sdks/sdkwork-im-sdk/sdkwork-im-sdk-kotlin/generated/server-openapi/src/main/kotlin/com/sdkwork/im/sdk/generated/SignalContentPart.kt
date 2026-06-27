package com.sdkwork.im.sdk.generated

data class SignalContentPart(
    val kind: String,
    val signalType: String,
    val schemaRef: String? = null,
    val payload: String
) : ContentPart
