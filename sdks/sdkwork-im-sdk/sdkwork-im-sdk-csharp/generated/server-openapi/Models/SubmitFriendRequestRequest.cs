using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SubmitFriendRequestRequest
    {
        public string TargetUserId { get; set; }
        public string? RequestMessage { get; set; }
    }
}
