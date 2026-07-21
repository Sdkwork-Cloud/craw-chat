using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class LagPageData
    {
        public List<LagItem> Items { get; set; }
        public PageInfo PageInfo { get; set; }
    }
}
