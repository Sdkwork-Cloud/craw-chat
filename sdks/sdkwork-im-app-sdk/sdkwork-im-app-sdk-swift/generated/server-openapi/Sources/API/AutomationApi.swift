import Foundation

public class AutomationApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Start an agent response stream
    public func agentResponsesCreate(body: StartAgentResponseRequest) async throws -> StreamSession? {
        return try await client.post(ApiPaths.appPath("/automation/agent_responses"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamSession.self)
    }

    /// Complete an agent response stream
    public func agentResponsesComplete(streamId: String, body: CompleteAgentResponseRequest) async throws -> StreamSession? {
        return try await client.post(ApiPaths.appPath("/automation/agent_responses/\(serializePathParameter(streamId, PathParameterSpec(name: "streamId", style: "simple", explode: false)))/complete"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamSession.self)
    }

    /// Append a frame to an agent response stream
    public func agentResponsesFramesCreate(streamId: String, body: AppendAgentResponseDeltaRequest) async throws -> StreamFrame? {
        return try await client.post(ApiPaths.appPath("/automation/agent_responses/\(serializePathParameter(streamId, PathParameterSpec(name: "streamId", style: "simple", explode: false)))/frames"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamFrame.self)
    }

    /// Request an agent tool call
    public func agentToolCallsCreate(body: RequestAgentToolCallRequest) async throws -> AgentToolCall? {
        return try await client.post(ApiPaths.appPath("/automation/agent_tool_calls"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: AgentToolCall.self)
    }

    /// Request an automation execution
    public func executionsCreate(body: RequestAutomationExecution) async throws -> AutomationExecutionRequestResponse? {
        return try await client.post(ApiPaths.appPath("/automation/executions"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: AutomationExecutionRequestResponse.self)
    }

    /// Get an automation execution
    public func executionsRetrieve(executionId: String) async throws -> AutomationExecution? {
        return try await client.get(ApiPaths.appPath("/automation/executions/\(serializePathParameter(executionId, PathParameterSpec(name: "executionId", style: "simple", explode: false)))"), responseType: AutomationExecution.self)
    }

    /// Complete an agent tool call
    public func agentToolCallsComplete(executionId: String, toolCallId: String, body: CompleteAgentToolCallRequest) async throws -> AgentToolCall? {
        return try await client.post(ApiPaths.appPath("/automation/executions/\(serializePathParameter(executionId, PathParameterSpec(name: "executionId", style: "simple", explode: false)))/agent_tool_calls/\(serializePathParameter(toolCallId, PathParameterSpec(name: "toolCallId", style: "simple", explode: false)))/complete"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: AgentToolCall.self)
    }

    private struct PathParameterSpec {
        let name: String
        let style: String
        let explode: Bool
    }

    private func serializePathParameter(_ value: Any?, _ spec: PathParameterSpec) -> String {
        guard let value else { return "" }
        let style = spec.style.isEmpty ? "simple" : spec.style
        if let array = value as? [Any] {
            return serializePathArray(spec.name, array, style, spec.explode)
        }
        if let object = value as? [String: Any] {
            return serializePathObject(spec.name, object, style, spec.explode)
        }
        return pathPrimitivePrefix(spec.name, style) + pathEncode(String(describing: value))
    }

    private func serializePathArray(_ name: String, _ values: [Any], _ style: String, _ explode: Bool) -> String {
        let serialized = values.map { pathEncode(String(describing: $0)) }
        if serialized.isEmpty { return pathPrefix(name, style) }
        if style == "matrix" {
            if explode {
                return serialized.map { ";\(name)=\($0)" }.joined()
            }
            return ";\(name)=" + serialized.joined(separator: ",")
        }
        let separator = explode ? "." : ","
        return pathPrefix(name, style) + serialized.joined(separator: separator)
    }

    private func serializePathObject(_ name: String, _ values: [String: Any], _ style: String, _ explode: Bool) -> String {
        var entries: [String] = []
        var exploded: [String] = []
        for (key, value) in values {
            let escapedKey = pathEncode(key)
            let escapedValue = pathEncode(String(describing: value))
            if explode {
                if style == "matrix" {
                    exploded.append(";\(escapedKey)=\(escapedValue)")
                } else {
                    exploded.append("\(escapedKey)=\(escapedValue)")
                }
            } else {
                entries.append(escapedKey)
                entries.append(escapedValue)
            }
        }
        if style == "matrix" {
            if explode {
                return exploded.joined()
            }
            return ";\(name)=" + entries.joined(separator: ",")
        }
        if explode {
            let separator = style == "label" ? "." : ","
            return pathPrefix(name, style) + exploded.joined(separator: separator)
        }
        return pathPrefix(name, style) + entries.joined(separator: ",")
    }

    private func pathPrefix(_ name: String, _ style: String) -> String {
        if style == "label" { return "." }
        if style == "matrix" { return ";\(name)" }
        return ""
    }

    private func pathPrimitivePrefix(_ name: String, _ style: String) -> String {
        style == "matrix" ? ";\(name)=" : pathPrefix(name, style)
    }

    private func pathEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? value
    }


}
