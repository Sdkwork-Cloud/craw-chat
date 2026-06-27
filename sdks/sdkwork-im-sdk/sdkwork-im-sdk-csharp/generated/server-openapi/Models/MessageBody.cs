using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageBody
    {
        public string? Text { get; set; }
        public List<ContentPart> Parts { get; set; }
        public MessageReplyReference? ReplyTo { get; set; }
        public Dictionary<string, object>? RenderHints { get; set; }
        public string? Summary { get; set; }
        public Dictionary<string, object>? Metadata { get; set; }
    }
}
