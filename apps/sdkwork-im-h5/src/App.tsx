import { useEffect } from 'react';
import { HashRouter } from 'react-router-dom';
import { useAudioStore } from '@sdkwork/im-h5-core';
import { AuthGate } from './AuthGate';
import { ImApp, IM_APP_HOME_PATH } from './ImApp';

export { IM_APP_HOME_PATH };

export default function App() {
  const initAudio = useAudioStore((s) => s.initAudio);

  useEffect(() => {
    initAudio();
  }, [initAudio]);

  return (
    <HashRouter>
      <AuthGate>
        <ImApp />
      </AuthGate>
    </HashRouter>
  );
}
