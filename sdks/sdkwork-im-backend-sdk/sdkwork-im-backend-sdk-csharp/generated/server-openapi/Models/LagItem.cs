using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class LagItem
    {
        public string Component { get; set; }
        public string ScopeId { get; set; }
        public string CurrentOffset { get; set; }
        public string CommittedOffset { get; set; }
        public string Lag { get; set; }
    }
}
