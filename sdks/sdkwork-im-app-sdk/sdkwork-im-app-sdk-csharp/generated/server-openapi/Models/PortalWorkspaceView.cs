using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalWorkspaceView
    {
        public string Name { get; set; }
        public string Slug { get; set; }
        public string Environment { get; set; }
        public string? Tier { get; set; }
        public string? Region { get; set; }
        public string? SupportPlan { get; set; }
        public string? Seats { get; set; }
        public string? ActiveBrands { get; set; }
    }
}
