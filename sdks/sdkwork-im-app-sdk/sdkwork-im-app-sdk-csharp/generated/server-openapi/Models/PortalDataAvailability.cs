using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class PortalDataAvailability
    {
        public string State { get; set; }
        public string Source { get; set; }
        public bool Complete { get; set; }
        public string? Reason { get; set; }
    }
}
