import { useCallback, useEffect, useState } from "react";
import { ChevronLeft, UserPlus } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { Avatar, IconButton, showToast } from "@sdkwork/im-h5-commons";
import type { FriendRequest } from "@sdkwork/im-h5-core/sdk";
import { subscribeScopeEvents } from "@sdkwork/im-h5-core/realtime";
import { useAppStore } from "@sdkwork/im-h5-core";

import {
  ContactService,
  FRIEND_REQUEST_REALTIME_EVENT_TYPES,
  SDKWORK_IM_H5_FRIEND_REQUESTS_CHANGED_EVENT,
} from "../services/ContactService";

export function NewFriends() {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const currentUser = useAppStore((state) => state.currentUser);
  const [items, setItems] = useState<FriendRequest[]>([]);
  const [nextCursor, setNextCursor] = useState<string>();
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [mutatingId, setMutatingId] = useState<string>();
  const [loadError, setLoadError] = useState(false);

  const load = useCallback(async (cursor?: string) => {
    cursor ? setLoadingMore(true) : setLoading(true);
    try {
      const page = await ContactService.listFriendRequests("incoming", cursor);
      setItems((previous) => mergeRequests(cursor ? previous : [], page.items));
      setNextCursor(page.hasMore ? page.nextCursor : undefined);
      setLoadError(false);
    } catch (error) {
      console.error(error);
      setLoadError(true);
    } finally {
      setLoading(false);
      setLoadingMore(false);
    }
  }, []);

  useEffect(() => {
    void load();
    const handleChanged = () => { void load(); };
    window.addEventListener(SDKWORK_IM_H5_FRIEND_REQUESTS_CHANGED_EVENT, handleChanged);
    const unsubscribeScope = currentUser?.id
      ? subscribeScopeEvents("user", currentUser.id, (event) => {
        const eventType = String(event.eventType ?? event.type ?? "");
        if (FRIEND_REQUEST_REALTIME_EVENT_TYPES.includes(eventType)) {
          void load();
        }
      }, FRIEND_REQUEST_REALTIME_EVENT_TYPES)
      : undefined;
    return () => {
      window.removeEventListener(SDKWORK_IM_H5_FRIEND_REQUESTS_CHANGED_EVENT, handleChanged);
      unsubscribeScope?.();
    };
  }, [load, currentUser?.id]);

  const mutate = async (request: FriendRequest, action: "accept" | "decline") => {
    if (mutatingId) return;
    setMutatingId(request.friendRequestId);
    try {
      if (action === "accept") {
        await ContactService.acceptFriendRequest(request.friendRequestId);
      } else {
        await ContactService.declineFriendRequest(request.friendRequestId);
      }
      setItems((previous) => previous.filter(
        (item) => item.friendRequestId !== request.friendRequestId,
      ));
      showToast(t(action === "accept" ? "contacts.request_accepted" : "contacts.request_declined"));
    } catch (error) {
      console.error(error);
      showToast(t("contacts.request_action_failed"));
    } finally {
      setMutatingId(undefined);
    }
  };

  return (
    <div className="flex h-full flex-col bg-bg-color">
      <header className="glass-header relative flex h-[56px] shrink-0 items-center justify-between px-1 pt-safe">
        <div className="z-10 flex flex-1 items-center">
          <IconButton
            icon={<ChevronLeft className="h-6 w-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <h2 className="pointer-events-none absolute inset-x-0 text-center text-[17px] font-medium text-text-main">
          {t("contacts.new_friends")}
        </h2>
        <button
          type="button"
          className="z-10 flex-1 pr-4 text-right text-[14px] font-medium text-primary-blue"
          onClick={() => navigate("/add-friend")}
        >
          {t("contacts.add_friend")}
        </button>
      </header>

      <div className="min-h-0 flex-1 overflow-y-auto">
        {loading && <p className="p-8 text-center text-[14px] text-text-sub">{t("common.loading", "Loading...")}</p>}
        {!loading && loadError && (
          <button type="button" className="w-full p-8 text-[14px] text-primary-blue" onClick={() => void load()}>
            {t("common.retry", "Retry")}
          </button>
        )}
        {!loading && !loadError && items.length === 0 && (
          <div className="flex flex-col items-center gap-3 p-12 text-text-sub">
            <UserPlus className="h-10 w-10" />
            <p className="text-[14px]">{t("contacts.no_friend_requests")}</p>
          </div>
        )}
        {items.map((request) => (
          <div key={request.friendRequestId} className="flex items-center gap-3 border-b border-border-color px-4 py-3">
            <Avatar fallback={request.requesterDisplayName || request.requesterUserId} size="md" />
            <div className="min-w-0 flex-1">
              <p className="truncate text-[16px] font-medium text-text-main">
                {request.requesterDisplayName || request.requesterUserId}
              </p>
              <p className="truncate text-[13px] text-text-sub">
                {request.requestMessage || t("contacts.friend_request_default")}
              </p>
            </div>
            <button
              type="button"
              disabled={mutatingId === request.friendRequestId}
              className="rounded-md bg-primary-blue px-3 py-1.5 text-[13px] font-medium text-white disabled:opacity-50"
              onClick={() => void mutate(request, "accept")}
            >
              {t("contacts.accept")}
            </button>
            <button
              type="button"
              disabled={mutatingId === request.friendRequestId}
              className="px-1 py-1.5 text-[13px] text-text-sub disabled:opacity-50"
              onClick={() => void mutate(request, "decline")}
            >
              {t("contacts.decline")}
            </button>
          </div>
        ))}
        {nextCursor && (
          <button type="button" disabled={loadingMore} className="h-12 w-full text-[14px] text-primary-blue" onClick={() => void load(nextCursor)}>
            {loadingMore ? t("common.loading", "Loading...") : t("common.load_more", "Load more")}
          </button>
        )}
      </div>
    </div>
  );
}

function mergeRequests(previous: readonly FriendRequest[], incoming: readonly FriendRequest[]): FriendRequest[] {
  const requests = new Map(previous.map((request) => [request.friendRequestId, request]));
  for (const request of incoming) requests.set(request.friendRequestId, request);
  return Array.from(requests.values());
}
