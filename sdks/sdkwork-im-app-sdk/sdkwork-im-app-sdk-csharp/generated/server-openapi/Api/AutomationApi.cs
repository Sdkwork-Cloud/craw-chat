using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.AppApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.AppApi.Generated.Api
{
    public class AutomationApi
    {
        private readonly SdkHttpClient _client;

        public AutomationApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Start an agent response stream
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.StreamSession?> AgentResponsesCreateAsync(Sdkwork.Im.AppApi.Generated.Models.StartAgentResponseRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.AppApi.Generated.Models.StreamSession>(ApiPaths.AppPath("/automation/agent_responses"), body, null, null, "application/json");
        }

        /// <summary>
        /// Complete an agent response stream
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.StreamSession?> AgentResponsesCompleteAsync(string streamId, Sdkwork.Im.AppApi.Generated.Models.CompleteAgentResponseRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.AppApi.Generated.Models.StreamSession>(ApiPaths.AppPath($"/automation/agent_responses/{SerializePathParameter(streamId, new PathParameterSpec("streamId", "simple", false))}/complete"), body, null, null, "application/json");
        }

        /// <summary>
        /// Append a frame to an agent response stream
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.StreamFrame?> AgentResponsesFramesCreateAsync(string streamId, Sdkwork.Im.AppApi.Generated.Models.AppendAgentResponseDeltaRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.AppApi.Generated.Models.StreamFrame>(ApiPaths.AppPath($"/automation/agent_responses/{SerializePathParameter(streamId, new PathParameterSpec("streamId", "simple", false))}/frames"), body, null, null, "application/json");
        }

        /// <summary>
        /// Request an agent tool call
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.AgentToolCall?> AgentToolCallsCreateAsync(Sdkwork.Im.AppApi.Generated.Models.RequestAgentToolCallRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.AppApi.Generated.Models.AgentToolCall>(ApiPaths.AppPath("/automation/agent_tool_calls"), body, null, null, "application/json");
        }

        /// <summary>
        /// Request an automation execution
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.AutomationExecutionRequestResponse?> ExecutionsCreateAsync(Sdkwork.Im.AppApi.Generated.Models.RequestAutomationExecution body)
        {
            return await _client.PostAsync<Sdkwork.Im.AppApi.Generated.Models.AutomationExecutionRequestResponse>(ApiPaths.AppPath("/automation/executions"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get an automation execution
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.AutomationExecution?> ExecutionsRetrieveAsync(string executionId)
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.AutomationExecution>(ApiPaths.AppPath($"/automation/executions/{SerializePathParameter(executionId, new PathParameterSpec("executionId", "simple", false))}"));
        }

        /// <summary>
        /// Complete an agent tool call
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.AgentToolCall?> AgentToolCallsCompleteAsync(string executionId, string toolCallId, Sdkwork.Im.AppApi.Generated.Models.CompleteAgentToolCallRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.AppApi.Generated.Models.AgentToolCall>(ApiPaths.AppPath($"/automation/executions/{SerializePathParameter(executionId, new PathParameterSpec("executionId", "simple", false))}/agent_tool_calls/{SerializePathParameter(toolCallId, new PathParameterSpec("toolCallId", "simple", false))}/complete"), body, null, null, "application/json");
        }

        private sealed record PathParameterSpec(string Name, string Style, bool Explode);

        private static string SerializePathParameter(object? value, PathParameterSpec spec)
        {
            if (value is null)
            {
                return string.Empty;
            }
            var style = string.IsNullOrWhiteSpace(spec.Style) ? "simple" : spec.Style;
            if (value is System.Collections.IDictionary dictionary)
            {
                return SerializePathObject(spec.Name, dictionary, style, spec.Explode);
            }
            if (value is System.Collections.IEnumerable enumerable && value is not string)
            {
                return SerializePathArray(spec.Name, enumerable, style, spec.Explode);
            }
            return PathPrimitivePrefix(spec.Name, style) + Uri.EscapeDataString(value.ToString() ?? string.Empty);
        }

        private static string SerializePathArray(string name, System.Collections.IEnumerable values, string style, bool explode)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(Uri.EscapeDataString(item.ToString() ?? string.Empty));
                }
            }
            if (serialized.Count == 0)
            {
                return PathPrefix(name, style);
            }
            if (style == "matrix")
            {
                if (explode)
                {
                    var parts = new List<string>();
                    foreach (var item in serialized)
                    {
                        parts.Add(";" + name + "=" + item);
                    }
                    return string.Join(string.Empty, parts);
                }
                return ";" + name + "=" + string.Join(",", serialized);
            }
            var separator = explode ? "." : ",";
            return PathPrefix(name, style) + string.Join(separator, serialized);
        }

        private static string SerializePathObject(string name, System.Collections.IDictionary values, string style, bool explode)
        {
            var entries = new List<string>();
            var exploded = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                var escapedKey = Uri.EscapeDataString(item.Key.ToString() ?? string.Empty);
                var escapedValue = Uri.EscapeDataString(item.Value.ToString() ?? string.Empty);
                if (explode)
                {
                    exploded.Add(style == "matrix" ? ";" + escapedKey + "=" + escapedValue : escapedKey + "=" + escapedValue);
                }
                else
                {
                    entries.Add(escapedKey);
                    entries.Add(escapedValue);
                }
            }
            if (style == "matrix")
            {
                return explode ? string.Join(string.Empty, exploded) : ";" + name + "=" + string.Join(",", entries);
            }
            if (explode)
            {
                var separator = style == "label" ? "." : ",";
                return PathPrefix(name, style) + string.Join(separator, exploded);
            }
            return PathPrefix(name, style) + string.Join(",", entries);
        }

        private static string PathPrefix(string name, string style)
        {
            return style switch
            {
                "label" => ".",
                "matrix" => ";" + name,
                _ => string.Empty,
            };
        }

        private static string PathPrimitivePrefix(string name, string style)
        {
            return style == "matrix" ? ";" + name + "=" : PathPrefix(name, style);
        }


    }
}
