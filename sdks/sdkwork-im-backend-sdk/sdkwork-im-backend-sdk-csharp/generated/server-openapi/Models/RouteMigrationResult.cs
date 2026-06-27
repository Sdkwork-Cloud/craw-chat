using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class RouteMigrationResult
    {
        public int? MigratedRouteCount { get; set; }
        public string? SourceDrainStatus { get; set; }
        public string? SourceNodeId { get; set; }
        public string? SourceRebalanceState { get; set; }
        public string? TargetDrainStatus { get; set; }
        public string? TargetNodeId { get; set; }
        public string? TargetRebalanceState { get; set; }
    }
}
