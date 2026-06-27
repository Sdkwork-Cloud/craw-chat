package com.sdkwork.im.app.api.generated

data class AutomationExecution(
    val tenantId: String? = null,
    val principalId: String? = null,
    val principalKind: String? = null,
    val executionId: String? = null,
    val triggerType: String? = null,
    val targetKind: String? = null,
    val targetRef: String? = null,
    val inputPayload: String? = null,
    val outputPayload: String? = null,
    val state: String? = null,
    val retryCount: Int? = null,
    val requestedAt: String? = null,
    val completedAt: String? = null,
    val failureReason: String? = null
)
