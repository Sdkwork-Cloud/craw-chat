import { getImSdkClient } from "@sdkwork/im-h5-core";
import type { PostedMessageResponse, TimelineResponse } from "@sdkwork/im-sdk";

export async function fetchConversationTimeline(
  conversationId: string,
  limit = 50,
): Promise<TimelineResponse> {
  return getImSdkClient().conversations.listMessages(conversationId, { limit });
}

export async function sendConversationText(
  conversationId: string,
  text: string,
): Promise<PostedMessageResponse> {
  return getImSdkClient().conversations.postText(conversationId, text.trim());
}
