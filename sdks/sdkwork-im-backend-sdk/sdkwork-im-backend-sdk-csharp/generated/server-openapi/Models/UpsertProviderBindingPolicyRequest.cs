using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class UpsertProviderBindingPolicyRequest
    {
        public string? Domain { get; set; }
        public int? ExpectedBaseVersion { get; set; }
        public string? PluginId { get; set; }
        public string? TenantId { get; set; }
    }
}
