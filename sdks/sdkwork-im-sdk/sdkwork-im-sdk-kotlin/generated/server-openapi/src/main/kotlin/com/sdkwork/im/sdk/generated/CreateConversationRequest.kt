package com.sdkwork.im.sdk.generated

data class CreateConversationRequest(
    val conversationId: String? = null,
    val conversationType: String? = null,
    val groupName: String? = null,
    val clientRequestKey: String? = null,
    val initializeKnowledgebase: Boolean? = null,
    val memberUserIds: List<String>? = null,
    val agentAssignments: List<ConversationAgentAssignment>? = null,
    val policyVersion: String? = null,
    val capabilityFlags: List<String>? = null,
    val historyVisibility: String? = null,
    val retentionPolicyRef: String? = null
)
