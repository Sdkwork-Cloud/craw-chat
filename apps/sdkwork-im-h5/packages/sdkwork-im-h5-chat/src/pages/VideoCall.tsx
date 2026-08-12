import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import { RtcCallPage } from "@sdkwork/rtc-h5-call";

import { ChatService } from "../services/ChatService";
import { imH5CallSignaling } from "../services/imH5CallSignaling";
import { consumePendingIncomingCall } from "../services/incomingCallWatcher";

interface CallContact {
  name?: string;
  avatar?: string;
}

/**
 * Video call surface.
 *
 * The full-screen call UI and media runtime live in `@sdkwork/rtc-h5-call`
 * (RTC authority); this page injects the IM H5 signaling adapter so the call
 * workflow runs on IM call signaling (`/im/v3/api/calls/*` + WebSocket).
 * Visual entry (chat header buttons) and route (`/call/video/:id`) are
 * unchanged. When signaling is unavailable the page is fail-closed and shows
 * the typed unavailable state — it never simulates a connection.
 */
export const VideoCall: React.FC = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const conversationId = id ?? "";
  const [pending] = useState(() => consumePendingIncomingCall());
  const isIncoming = Boolean(pending);
  const [contact, setContact] = useState<CallContact>({});

  useEffect(() => {
    let active = true;
    if (!conversationId) {
      return () => {
        active = false;
      };
    }
    void ChatService.getChatById(conversationId)
      .then((chat) => {
        if (!active) {
          return;
        }
        const peer = chat?.participants?.[0];
        setContact({ name: peer?.name, avatar: peer?.avatar });
      })
      .catch(() => {
        // Contact profile is decorative; the call proceeds without it.
      });
    return () => {
      active = false;
    };
  }, [conversationId]);

  return (
    <RtcCallPage
      type="video"
      mode={isIncoming ? "incoming" : "outgoing"}
      conversationId={conversationId}
      targetName={contact.name}
      targetAvatar={contact.avatar}
      rtcSessionId={pending?.rtcSessionId}
      signaling={imH5CallSignaling}
      onExit={() => navigate(-1)}
    />
  );
};
