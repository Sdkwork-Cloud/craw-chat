package com.sdkwork.im.sdk.generated

data class PostMessageRequest(
    val text: String? = null,
    val parts: List<ContentPart>? = null,
    val replyTo: MessageReplyReference? = null,
    val clientMsgId: String? = null,
    val summary: String? = null,
    val renderHints: Map<String, Any>? = null
)
