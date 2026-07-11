package com.sdkwork.im.sdk.generated

data class EditMessageRequest(
    val text: String? = null,
    val parts: List<ContentPart>? = null,
    val replyTo: MessageReplyReference? = null,
    val summary: String? = null,
    val renderHints: Map<String, Any>? = null,
    val idempotencyKey: String? = null
)
