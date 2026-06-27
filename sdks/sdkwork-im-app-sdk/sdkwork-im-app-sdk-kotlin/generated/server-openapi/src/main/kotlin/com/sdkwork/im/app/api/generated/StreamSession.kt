package com.sdkwork.im.app.api.generated

data class StreamSession(
    val tenantId: String? = null,
    val streamId: String? = null,
    val streamType: String? = null,
    val scopeKind: String? = null,
    val scopeId: String? = null,
    val durabilityClass: String? = null,
    val orderingScope: String? = null,
    val schemaRef: String? = null,
    val state: String? = null,
    val lastFrameSeq: Int? = null,
    val lastCheckpointSeq: Int? = null,
    val resultMessageId: String? = null,
    val openedAt: String? = null,
    val closedAt: String? = null,
    val expiresAt: String? = null
)
