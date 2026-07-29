import assert from "node:assert/strict";
import test from "node:test";

import { CommunityCapabilityUnavailableError, CommunityService } from "./CommunityService";

test("community operations fail closed", async () => {
  const community = {
    coverImage: "",
    description: "Description",
    name: "Community",
    tags: [],
  };
  const group = {
    memberCount: 0,
    name: "Group",
    platform: "other" as const,
  };
  for (const operation of [
    () => CommunityService.getMembersByCommunity("community-id"),
    () => CommunityService.updateMemberRole("community-id", "member-id", "admin"),
    () => CommunityService.updateMemberStatus("community-id", "member-id", "muted"),
    () => CommunityService.removeMember("community-id", "member-id"),
    () => CommunityService.createCommunity(community),
    () => CommunityService.getCommunities(),
    () => CommunityService.getCommunityById("community-id"),
    () => CommunityService.joinCommunity("community-id"),
    () => CommunityService.getPostsByCommunity("community-id"),
    () => CommunityService.createPost("community-id", "Content"),
    () => CommunityService.addComment("community-id", "post-id", "Comment"),
    () => CommunityService.toggleLikePost("community-id", "post-id"),
    () => CommunityService.getResourcesByCommunity("community-id"),
    () => CommunityService.getGroupsByCommunity("community-id"),
    () => CommunityService.createGroup("community-id", group),
    () => CommunityService.updateGroup("community-id", "group-id", { name: "Updated" }),
    () => CommunityService.updateCommunity("community-id", { name: "Updated" }),
    () => CommunityService.deleteGroup("community-id", "group-id"),
  ]) {
    await assert.rejects(operation, CommunityCapabilityUnavailableError);
  }
});
