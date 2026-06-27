using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ContactPreferencesView
    {
        public string TenantId { get; set; }
        public string OwnerUserId { get; set; }
        public string TargetUserId { get; set; }
        public bool IsStarred { get; set; }
        public string Remark { get; set; }
        public bool IsBlocked { get; set; }
        public string UpdatedAt { get; set; }
    }
}
