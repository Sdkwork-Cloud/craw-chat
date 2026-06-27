using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class EditMessageRequest
    {
        public string? Text { get; set; }
        public List<ContentPart>? Parts { get; set; }
        public MessageReplyReference? ReplyTo { get; set; }
    }
}
