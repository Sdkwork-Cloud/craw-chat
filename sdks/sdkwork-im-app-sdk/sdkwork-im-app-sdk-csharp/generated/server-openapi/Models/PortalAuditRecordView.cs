using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalAuditRecordView
    {
        public string RecordId { get; set; }
        public string Action { get; set; }
        public string ActorId { get; set; }
        public string RecordedAt { get; set; }
        public string Severity { get; set; }
    }
}
