using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SpaceChannelAccessRuleView
    {
        public string RuleId { get; set; }
        public string ChannelId { get; set; }
        public string RuleType { get; set; }
        public string? PrincipalKind { get; set; }
        public string? PrincipalId { get; set; }
        public string Permission { get; set; }
        public string CreatedAt { get; set; }
    }
}
