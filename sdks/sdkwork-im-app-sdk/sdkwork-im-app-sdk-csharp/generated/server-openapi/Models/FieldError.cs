using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class FieldError
    {
        public string Field { get; set; }
        public string Message { get; set; }
        public int? Code { get; set; }
    }
}
