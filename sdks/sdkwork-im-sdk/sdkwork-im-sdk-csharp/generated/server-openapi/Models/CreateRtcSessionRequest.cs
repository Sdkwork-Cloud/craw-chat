using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateRtcSessionRequest
    {
        public string RtcSessionId { get; set; }
        public string? ConversationId { get; set; }
        public string RtcMode { get; set; }
    }
}
