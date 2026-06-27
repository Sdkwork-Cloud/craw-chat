import Foundation

public class SpacesApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Create a space
    public func create(body: SpaceCreateRequest) async throws -> SpaceView? {
        return try await client.post(ApiPaths.imPath("/spaces"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceView.self)
    }

    /// List spaces
    public func list() async throws -> SpaceListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces"), responseType: SpaceListResponse.self)
    }

    /// Get a space
    public func get_(spaceId: String) async throws -> SpaceView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))"), responseType: SpaceView.self)
    }

    /// Update a space
    public func update(spaceId: String, body: SpaceUpdateRequest) async throws -> SpaceView? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceView.self)
    }

    /// Delete a space
    public func delete(spaceId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))"))
    }

    /// List spaces members
    public func membersList(spaceId: String) async throws -> SpaceMemberListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members"), responseType: SpaceMemberListResponse.self)
    }

    /// Create spaces members
    public func membersCreate(spaceId: String, body: SpaceMemberCreateRequest) async throws -> SpaceMemberView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceMemberView.self)
    }

    /// Get spaces members
    public func membersGet(spaceId: String, userId: String) async throws -> SpaceMemberView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), responseType: SpaceMemberView.self)
    }

    /// Update spaces members
    public func membersUpdate(spaceId: String, userId: String, body: SpaceMemberUpdateRequest) async throws -> SpaceMemberView? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceMemberView.self)
    }

    /// Delete spaces members
    public func membersDelete(spaceId: String, userId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"))
    }

    /// List spaces groups
    public func groupsList(spaceId: String) async throws -> SpaceGroupListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups"), responseType: SpaceGroupListResponse.self)
    }

    /// Create spaces groups
    public func groupsCreate(spaceId: String, body: SpaceGroupCreateRequest) async throws -> SpaceGroupView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceGroupView.self)
    }

    /// Get spaces groups
    public func groupsGet(spaceId: String, groupId: String) async throws -> SpaceGroupView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"), responseType: SpaceGroupView.self)
    }

    /// Update spaces groups
    public func groupsUpdate(spaceId: String, groupId: String, body: SpaceGroupUpdateRequest) async throws -> SpaceGroupView? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceGroupView.self)
    }

    /// Delete spaces groups
    public func groupsDelete(spaceId: String, groupId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"))
    }

    /// List spaces groups members
    public func groupsMembersList(spaceId: String, groupId: String) async throws -> SpaceGroupMemberListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members"), responseType: SpaceGroupMemberListResponse.self)
    }

    /// Create spaces groups members
    public func groupsMembersCreate(spaceId: String, groupId: String, body: SpaceGroupMemberCreateRequest) async throws -> SpaceGroupMemberView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceGroupMemberView.self)
    }

    /// Get spaces groups members
    public func groupsMembersGet(spaceId: String, groupId: String, userId: String) async throws -> SpaceGroupMemberView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), responseType: SpaceGroupMemberView.self)
    }

    /// Update spaces groups members
    public func groupsMembersUpdate(spaceId: String, groupId: String, userId: String, body: SpaceGroupMemberUpdateRequest) async throws -> Void {
        _ = try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// Delete spaces groups members
    public func groupsMembersDelete(spaceId: String, groupId: String, userId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"))
    }

    /// List spaces channels
    public func channelsList(spaceId: String) async throws -> SpaceChannelListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels"), responseType: SpaceChannelListResponse.self)
    }

    /// Create spaces channels
    public func channelsCreate(spaceId: String, body: SpaceChannelCreateRequest) async throws -> SpaceChannelView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceChannelView.self)
    }

    /// Get spaces channels
    public func channelsGet(spaceId: String, channelId: String) async throws -> SpaceChannelView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"), responseType: SpaceChannelView.self)
    }

    /// Update spaces channels
    public func channelsUpdate(spaceId: String, channelId: String, body: SpaceChannelUpdateRequest) async throws -> SpaceChannelView? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceChannelView.self)
    }

    /// Delete spaces channels
    public func channelsDelete(spaceId: String, channelId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"))
    }

    /// List spaces channels access Rules
    public func channelsAccessRulesList(spaceId: String, channelId: String) async throws -> SpaceChannelAccessRuleListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/access_rules"), responseType: SpaceChannelAccessRuleListResponse.self)
    }

    /// Create spaces channels access Rules
    public func channelsAccessRulesCreate(spaceId: String, channelId: String, body: SpaceChannelAccessRuleCreateRequest) async throws -> SpaceChannelAccessRuleView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/access_rules"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceChannelAccessRuleView.self)
    }

    /// Delete spaces channels access Rules
    public func channelsAccessRulesDelete(spaceId: String, channelId: String, ruleId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/access_rules/\(serializePathParameter(ruleId, PathParameterSpec(name: "ruleId", style: "simple", explode: false)))"))
    }

    /// List spaces invites
    public func invitesList(spaceId: String) async throws -> SpaceInviteListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites"), responseType: SpaceInviteListResponse.self)
    }

    /// Create spaces invites
    public func invitesCreate(spaceId: String, body: SpaceInviteCreateRequest) async throws -> SpaceInviteView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceInviteView.self)
    }

    /// Get spaces invites
    public func invitesGet(spaceId: String, inviteCode: String) async throws -> SpaceInviteView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites/\(serializePathParameter(inviteCode, PathParameterSpec(name: "inviteCode", style: "simple", explode: false)))"), responseType: SpaceInviteView.self)
    }

    /// Revoke spaces invites
    public func invitesRevoke(spaceId: String, inviteCode: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites/\(serializePathParameter(inviteCode, PathParameterSpec(name: "inviteCode", style: "simple", explode: false)))"))
    }

    /// Accept spaces invites
    public func invitesAccept(spaceId: String, inviteCode: String) async throws -> Void {
        _ = try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites/\(serializePathParameter(inviteCode, PathParameterSpec(name: "inviteCode", style: "simple", explode: false)))/accept"), body: nil)
    }

    /// List spaces bans
    public func bansList(spaceId: String) async throws -> SpaceBanListResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans"), responseType: SpaceBanListResponse.self)
    }

    /// Create spaces bans
    public func bansCreate(spaceId: String, body: SpaceBanCreateRequest) async throws -> SpaceBanView? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpaceBanView.self)
    }

    /// Get spaces bans
    public func bansGet(spaceId: String, userId: String) async throws -> SpaceBanView? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), responseType: SpaceBanView.self)
    }

    /// Delete spaces bans
    public func bansDelete(spaceId: String, userId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"))
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
