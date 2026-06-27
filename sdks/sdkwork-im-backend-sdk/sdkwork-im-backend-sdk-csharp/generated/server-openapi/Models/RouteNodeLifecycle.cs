using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class RouteNodeLifecycle
    {
        public string? DrainStatus { get; set; }
        public string? NodeId { get; set; }
        public int? OwnedRouteCount { get; set; }
        public string? RebalanceState { get; set; }
    }
}
