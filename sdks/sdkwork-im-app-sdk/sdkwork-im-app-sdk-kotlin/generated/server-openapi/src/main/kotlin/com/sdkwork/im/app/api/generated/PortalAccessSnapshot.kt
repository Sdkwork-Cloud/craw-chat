package com.sdkwork.im.app.api.generated

data class PortalAccessSnapshot(
    val meta: PortalSnapshotMeta? = null,
    val availability: PortalDataAvailability? = null,
    val tenantId: String? = null,
    val principalId: String? = null,
    val recentItems: List<PortalAuditRecordView>? = null,
    val hasMore: Boolean? = null
)
