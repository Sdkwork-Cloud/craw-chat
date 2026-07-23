using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalAccessSnapshot
    {
        public PortalSnapshotMeta Meta { get; set; }
        public PortalDataAvailability Availability { get; set; }
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public List<PortalAuditRecordView> RecentItems { get; set; }
        public bool HasMore { get; set; }
    }
}
