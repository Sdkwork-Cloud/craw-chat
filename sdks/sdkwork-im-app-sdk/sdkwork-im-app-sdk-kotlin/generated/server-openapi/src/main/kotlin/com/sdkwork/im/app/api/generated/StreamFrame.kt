package com.sdkwork.im.app.api.generated

data class StreamFrame(
    val tenantId: String? = null,
    val streamId: String? = null,
    val streamType: String? = null,
    val scopeKind: String? = null,
    val scopeId: String? = null,
    val frameSeq: Int? = null,
    val frameType: String? = null,
    val schemaRef: String? = null,
    val encoding: String? = null,
    val payload: String? = null,
    val sender: Sender? = null,
    val attributes: Map<String, String>? = null,
    val occurredAt: String? = null
)
