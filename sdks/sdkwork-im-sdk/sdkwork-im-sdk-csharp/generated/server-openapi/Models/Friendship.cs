using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class Friendship
    {
        public string TenantId { get; set; }
        public string FriendshipId { get; set; }
        public string InitiatorUserId { get; set; }
        public string LeftUserId { get; set; }
        public string RightUserId { get; set; }
        public string UserHighId { get; set; }
        public string UserLowId { get; set; }
        public string Status { get; set; }
        public string CreatedAt { get; set; }
    }
}
