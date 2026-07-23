using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalRealtimeSnapshot
    {
        public PortalSnapshotMeta Meta { get; set; }
        public PortalDataAvailability Availability { get; set; }
        public PortalRealtimeMetrics? Metrics { get; set; }
    }
}
