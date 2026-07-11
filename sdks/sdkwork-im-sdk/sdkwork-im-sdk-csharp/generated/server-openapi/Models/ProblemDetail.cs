using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ProblemDetail
    {
        public string Type { get; set; }
        public string Title { get; set; }
        public int Status { get; set; }
        public string? Detail { get; set; }
        public string? Instance { get; set; }
        public int Code { get; set; }
        public string TraceId { get; set; }
        public string? I18nKey { get; set; }
        public string? Locale { get; set; }
        public List<FieldError>? Errors { get; set; }
    }
}
