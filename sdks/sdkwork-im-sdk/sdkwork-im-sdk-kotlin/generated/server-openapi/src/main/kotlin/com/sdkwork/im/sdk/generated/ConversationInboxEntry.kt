package com.sdkwork.im.sdk.generated

data class ConversationInboxEntry(
    val tenantId: String? = null,
    val conversationId: String? = null,
    val agentHandoff: Boolean? = null,
    val conversationType: String? = null,
    val displayName: String? = null,
    val avatarUrl: String? = null,
    val displaySource: String? = null,
    val peer: ConversationInboxPeerView? = null,
    val preferences: ConversationInboxPreferencesView? = null,
    val lastActivityAt: String? = null,
    val lastMessageId: String? = null,
    val lastSenderId: String? = null,
    val messageCount: Int? = null,
    val lastMessageSeq: Int? = null,
    val lastSummary: String? = null,
    val lastMessageAt: String? = null,
    val unreadCount: Int? = null
)
