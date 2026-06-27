using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class ProblemDetail
    {
        public string? Type { get; set; }
        public string? Title { get; set; }
        public int? Status { get; set; }
        public string? Detail { get; set; }
        public string? Code { get; set; }
        public string? Message { get; set; }
        public string? TraceId { get; set; }
        public bool? Retryable { get; set; }
    }
}
