using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class FriendRequest
    {
        public string TenantId { get; set; }
        public string RequestId { get; set; }
        public string RequesterUserId { get; set; }
        public string TargetUserId { get; set; }
        public string Status { get; set; }
        public string? RequestMessage { get; set; }
        public string CreatedAt { get; set; }
        public string UpdatedAt { get; set; }
    }
}
