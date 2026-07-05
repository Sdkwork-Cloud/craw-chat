using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class SdkWorkPageData
    {
        public List<Dictionary<string, object>> Items { get; set; }
        public PageInfo PageInfo { get; set; }
    }
}
