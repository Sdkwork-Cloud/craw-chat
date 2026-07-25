/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { HashRouter } from 'react-router-dom';
import { SdkworkSessionAuthBrowserRoot } from '@sdkwork/auth-pc-react';
import { getIamRuntime } from './bootstrap/iamRuntime';
import AppAuthGate from './AppAuthGate';
import ImApp, { IM_APP_HOME_PATH } from './ImApp';

export { IM_APP_HOME_PATH };

export default function App() {
  return (
    <HashRouter>
      <SdkworkSessionAuthBrowserRoot getRuntime={getIamRuntime}>
        <AppAuthGate>
          <ImApp />
        </AppAuthGate>
      </SdkworkSessionAuthBrowserRoot>
    </HashRouter>
  );
}
