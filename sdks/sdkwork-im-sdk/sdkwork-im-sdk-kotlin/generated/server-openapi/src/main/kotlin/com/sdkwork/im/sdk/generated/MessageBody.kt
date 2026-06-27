package com.sdkwork.im.sdk.generated

data class MessageBody(
    val text: String? = null,
    val parts: List<ContentPart>? = null,
    val replyTo: MessageReplyReference? = null,
    val renderHints: Map<String, Any>? = null,
    val summary: String? = null,
    val metadata: Map<String, Any>? = null
)
