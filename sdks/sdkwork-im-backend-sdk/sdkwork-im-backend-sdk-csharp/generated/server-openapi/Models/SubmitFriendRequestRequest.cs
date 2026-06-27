using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class SubmitFriendRequestRequest
    {
        public string? EventId { get; set; }
        public string? RequestId { get; set; }
        public string? RequestMessage { get; set; }
        public string? RequestedAt { get; set; }
        public string? RequesterUserId { get; set; }
        public string? TargetUserId { get; set; }
    }
}
