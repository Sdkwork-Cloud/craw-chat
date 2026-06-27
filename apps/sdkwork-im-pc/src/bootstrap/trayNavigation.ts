import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { resolveTauriListen, type TauriUnlisten } from '@sdkwork/im-pc-core/host/desktopHost';

export const TRAY_PENDING_SETTINGS_STORAGE_KEY = 'sdkwork-im-pc:pending-open-settings';

export function useTauriTrayNavigationBridge(): void {
  const navigate = useNavigate();

  useEffect(() => {
    const listen = resolveTauriListen();
    if (!listen) {
      return;
    }

    const unlisteners: TauriUnlisten[] = [];
    let disposed = false;

    void listen('sdkwork-im-pc://tray/open-chat', () => {
      navigate('/', { replace: false });
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    });

    void listen('sdkwork-im-pc://tray/open-settings', () => {
      sessionStorage.setItem(TRAY_PENDING_SETTINGS_STORAGE_KEY, '1');
      navigate('/', { replace: false });
      window.dispatchEvent(new CustomEvent('sdkwork-im-pc:open-settings'));
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    });

    void listen('sdkwork-im-pc://tray/show-active-call', () => {
      navigate('/', { replace: false });
      window.dispatchEvent(new CustomEvent('sdkwork-im-pc:show-active-call'));
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    });

    return () => {
      disposed = true;
      for (const unlisten of unlisteners.splice(0)) {
        unlisten();
      }
    };
  }, [navigate]);
}
