using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SocialUserSearchResult
    {
        public string TenantId { get; set; }
        public string UserId { get; set; }
        public string ChatId { get; set; }
        public string DisplayName { get; set; }
        public string RelationshipState { get; set; }
        public string? AvatarUrl { get; set; }
        public string? Email { get; set; }
        public string? Phone { get; set; }
        public Dictionary<string, object>? Metadata { get; set; }
    }
}
