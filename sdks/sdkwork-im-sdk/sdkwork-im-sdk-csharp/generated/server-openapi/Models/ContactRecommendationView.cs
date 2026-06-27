using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ContactRecommendationView
    {
        public string TenantId { get; set; }
        public string OwnerUserId { get; set; }
        public string TargetUserId { get; set; }
        public string RecommendationId { get; set; }
        public string? TargetConversationId { get; set; }
        public string CreatedAt { get; set; }
    }
}
