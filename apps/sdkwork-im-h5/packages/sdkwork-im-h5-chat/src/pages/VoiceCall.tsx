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
 * Voice call surface.
 *
 * Same composition as the video call page: full-screen call UI and media
 * runtime from `@sdkwork/rtc-h5-call` (RTC authority) with the IM H5
 * signaling adapter injected. Voice calls publish audio tracks only; the UI
 * renders the caller avatar surface instead of the video stage. Entry
 * (`/call/voice/:id`) and the chat header buttons are unchanged.
 */
export const VoiceCall: React.FC = () => {
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
      type="voice"
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
