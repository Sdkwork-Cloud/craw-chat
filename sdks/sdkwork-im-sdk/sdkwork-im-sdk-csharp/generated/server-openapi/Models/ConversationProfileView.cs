using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationProfileView
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string DisplayName { get; set; }
        public string AvatarUrl { get; set; }
        public string Notice { get; set; }
        public string UpdatedAt { get; set; }
        public string? UpdatedByPrincipalKind { get; set; }
        public string? UpdatedByPrincipalId { get; set; }
    }
}
