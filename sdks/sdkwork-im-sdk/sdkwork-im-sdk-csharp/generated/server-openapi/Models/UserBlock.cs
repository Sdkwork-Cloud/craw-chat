using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class UserBlock
    {
        public string TenantId { get; set; }
        public string BlockId { get; set; }
        public string BlockerUserId { get; set; }
        public string BlockedUserId { get; set; }
        public string Scope { get; set; }
        public string Status { get; set; }
        public string? DirectChatId { get; set; }
        public string? ExpiresAt { get; set; }
        public string CreatedAt { get; set; }
        public string UpdatedAt { get; set; }
    }
}
