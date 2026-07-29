import React, { useEffect, useMemo, useRef, useState } from "react";
import { Check, ChevronLeft, FileCheck, Loader2, Search } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import {
  appendBoundedUnique,
  NOTARY_CLIENT_WINDOW_LIMIT,
  notaryService,
  type NotaryStaffMember,
} from "../services/notaryService";
import { notaryDraftSession } from "../state/notaryDraftSession";

const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ#".split("");

export const NotarySearchList: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [selectionOpen] = useState(() =>
    notaryDraftSession.isNotarySelectionOpen(),
  );
  const [selectedNotaryId] = useState(
    () => notaryDraftSession.getDraft().selectedNotary,
  );
  const [searchQuery, setSearchQuery] = useState("");
  const [notaries, setNotaries] = useState<NotaryStaffMember[]>([]);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const requestSequence = useRef(0);

  useEffect(() => {
    if (!selectionOpen) {
      navigate("/notary/create", { replace: true });
    }
  }, [navigate, selectionOpen]);

  useEffect(() => {
    if (!selectionOpen) {
      return undefined;
    }
    const sequence = requestSequence.current + 1;
    requestSequence.current = sequence;
    setLoading(true);
    setLoadError(false);
    const timeout = window.setTimeout(() => {
      void notaryService.getNotarySearchList(searchQuery).then(
        (page) => {
          if (requestSequence.current === sequence) {
            setNotaries(page.staff);
            setNextCursor(page.pageInfo.hasMore ? page.pageInfo.nextCursor : undefined);
          }
        },
        () => {
          if (requestSequence.current === sequence) {
            setNotaries([]);
            setNextCursor(undefined);
            setLoadError(true);
          }
        },
      ).finally(() => {
        if (requestSequence.current === sequence) {
          setLoading(false);
        }
      });
    }, 250);
    return () => window.clearTimeout(timeout);
  }, [searchQuery, selectionOpen]);

  const loadMore = async () => {
    if (!nextCursor || loading || notaries.length >= NOTARY_CLIENT_WINDOW_LIMIT) {
      return;
    }
    setLoading(true);
    setLoadError(false);
    try {
      const page = await notaryService.getNotarySearchList(searchQuery, nextCursor);
      const merged = appendBoundedUnique(
        notaries,
        page.staff,
        (staff) => staff.id,
      );
      setNotaries(merged);
      setNextCursor(
        page.pageInfo.hasMore && merged.length < NOTARY_CLIENT_WINDOW_LIMIT
          ? page.pageInfo.nextCursor
          : undefined,
      );
    } catch {
      setLoadError(true);
    } finally {
      setLoading(false);
    }
  };

  const groupedNotaries = useMemo(() => {
    const groups = new Map<string, NotaryStaffMember[]>();
    for (const notary of notaries) {
      const group = groups.get(notary.initial) ?? [];
      group.push(notary);
      groups.set(notary.initial, group);
    }
    return [...groups.entries()]
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([initial, staff]) => ({ initial, staff }));
  }, [notaries]);

  const handleSelect = (notary: NotaryStaffMember) => {
    notaryDraftSession.selectNotary(notary);
    navigate(-1);
  };

  const handleBack = () => {
    notaryDraftSession.closeNotarySelection();
    navigate(-1);
  };

  const scrollToSection = (letter: string) => {
    document.getElementById(`section-${letter}`)?.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });
  };

  return (
    <div className="fixed inset-0 z-[100] flex h-full flex-col bg-bg-color">
      <header className="glass-header sticky top-0 z-10 flex h-[44px] shrink-0 items-center justify-between px-1 pt-safe">
        <div className="z-10 flex flex-1 items-center">
          <IconButton
            icon={<ChevronLeft className="h-6 w-6 text-text-main" strokeWidth={2.5} />}
            onClick={handleBack}
          />
        </div>
        <div className="pointer-events-none flex items-center justify-center text-[17px] font-bold text-text-main">
          {t("notary.search.title")}
        </div>
        <div className="flex-1" />
      </header>

      <div className="shrink-0 bg-bg-color px-4 py-3">
        <div className="flex items-center rounded-xl bg-input-bg px-3 py-2 text-[15px]">
          <Search className="mr-2 h-5 w-5 shrink-0 text-text-sub" />
          <input
            type="search"
            placeholder={t("notary.search.placeholder")}
            value={searchQuery}
            onChange={(event) => setSearchQuery(event.target.value)}
            className="flex-1 border-none bg-transparent text-text-main outline-none placeholder-text-sub"
          />
        </div>
      </div>

      <div className="relative flex-1 overflow-y-auto bg-[#F4F6F9] dark:bg-black">
        {groupedNotaries.map((group) => (
          <div key={group.initial} id={`section-${group.initial}`}>
            <div className="sticky top-0 z-10 bg-[#F4F6F9] px-4 py-1.5 text-[13px] font-bold text-text-sub dark:bg-black">
              {group.initial}
            </div>
            <div className="bg-chat-other-bg">
              {group.staff.map((notary, index) => (
                <button
                  type="button"
                  key={notary.id}
                  onClick={() => handleSelect(notary)}
                  className={cn(
                    "flex w-full items-center py-3 pl-4 text-left active:bg-active-bg",
                    selectedNotaryId === notary.id && "bg-primary-blue/5",
                  )}
                >
                  <div className="relative mr-3 flex h-10 w-10 shrink-0 items-center justify-center rounded-md bg-primary-blue text-white">
                    <FileCheck className="h-5 w-5" />
                    {notary.active && (
                      <div className="absolute -bottom-1 -right-1 h-3 w-3 rounded-full border-2 border-bg-color bg-green-500" />
                    )}
                  </div>
                  <div className={cn(
                    "flex min-w-0 flex-1 items-center justify-between pb-3 pr-4",
                    index !== group.staff.length - 1 && "border-b border-border-color",
                  )}>
                    <div className="min-w-0 flex-1">
                      <span className="block truncate text-[16px] font-medium text-text-main">
                        {notary.name}
                      </span>
                      <span className="block truncate text-[13px] text-text-sub">
                        {notary.organization}
                      </span>
                    </div>
                    {selectedNotaryId === notary.id && (
                      <Check className="ml-2 h-5 w-5 shrink-0 text-primary-blue" />
                    )}
                  </div>
                </button>
              ))}
            </div>
          </div>
        ))}

        <div className="flex min-h-16 items-center justify-center py-4">
          {loading ? (
            <Loader2 className="h-5 w-5 animate-spin text-text-sub" />
          ) : loadError ? (
            <span className="text-[13px] text-red-500">
              {t("notary.search.load_failed", "Unable to load notaries")}
            </span>
          ) : nextCursor ? (
            <button
              type="button"
              className="text-[13px] font-medium text-primary-blue"
              onClick={() => void loadMore()}
            >
              {t("notary.search.load_more", "Load more")}
            </button>
          ) : notaries.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12 text-text-sub">
              <Search className="mb-3 h-12 w-12 opacity-20" />
              <p>{t("notary.search.not_found")}</p>
            </div>
          ) : null}
        </div>
      </div>

      {!searchQuery && (
        <div className="absolute right-1 top-1/2 z-20 flex -translate-y-1/2 flex-col items-center p-1">
          {ALPHABET.map((letter) => (
            <button
              type="button"
              key={letter}
              onClick={() => scrollToSection(letter)}
              className="py-[1.5px] text-[10px] font-medium text-text-sub"
              aria-label={letter}
            >
              {letter}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};
