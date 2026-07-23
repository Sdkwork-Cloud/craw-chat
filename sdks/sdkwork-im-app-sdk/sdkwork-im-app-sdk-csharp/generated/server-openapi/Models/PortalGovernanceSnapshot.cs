using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalGovernanceSnapshot
    {
        public PortalSnapshotMeta Meta { get; set; }
        public PortalDataAvailability Availability { get; set; }
        public string SampledEventCount { get; set; }
        public PortalGovernanceRiskSample RiskSample { get; set; }
    }
}
