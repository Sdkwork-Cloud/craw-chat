using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalRealtimeMetrics
    {
        public string ClientRouteWindowCount { get; set; }
        public string PendingEventCount { get; set; }
        public string MaxClientRouteWindowEventCount { get; set; }
        public string ClientRouteWindowCapacity { get; set; }
        public int MaxClientRouteWindowUsagePermille { get; set; }
        public string CapacityTrimmedEventCount { get; set; }
        public string? OldestPendingOccurredAt { get; set; }
    }
}
