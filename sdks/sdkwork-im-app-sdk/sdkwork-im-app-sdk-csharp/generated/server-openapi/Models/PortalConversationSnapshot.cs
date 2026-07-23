using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalConversationSnapshot
    {
        public PortalSnapshotMeta Meta { get; set; }
        public PortalDataAvailability Availability { get; set; }
        public PortalConversationOperationalMetrics? Metrics { get; set; }
    }
}
