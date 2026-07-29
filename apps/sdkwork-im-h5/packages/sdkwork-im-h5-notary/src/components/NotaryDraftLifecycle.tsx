import { useEffect } from "react";
import { useLocation } from "react-router";

import { notaryDraftSession } from "../state/notaryDraftSession";

const NOTARY_DRAFT_PATHS = new Set([
  "/notary/create",
  "/notary/search",
  "/notary/add-party",
]);

export function NotaryDraftLifecycle() {
  const { pathname } = useLocation();

  useEffect(() => {
    if (pathname !== "/notary/add-party") {
      notaryDraftSession.closePartyEditor();
    }
    if (pathname !== "/notary/search") {
      notaryDraftSession.closeNotarySelection();
    }
    if (!NOTARY_DRAFT_PATHS.has(pathname)) {
      notaryDraftSession.reset();
    }
  }, [pathname]);

  return null;
}
