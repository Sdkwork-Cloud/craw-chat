using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class AgentSubject
    {
        public string? AgentId { get; set; }
        public string? SessionId { get; set; }
        public Dictionary<string, string>? Metadata { get; set; }
    }
}
