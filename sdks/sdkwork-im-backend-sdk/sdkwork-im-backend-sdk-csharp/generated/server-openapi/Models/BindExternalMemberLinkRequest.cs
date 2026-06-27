using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class BindExternalMemberLinkRequest
    {
        public string? ConnectionId { get; set; }
        public string? EventId { get; set; }
        public string? ExternalDisplayName { get; set; }
        public string? ExternalMemberId { get; set; }
        public string? LinkId { get; set; }
        public string? LinkedAt { get; set; }
        public string? LocalActorId { get; set; }
        public string? LocalActorKind { get; set; }
    }
}
