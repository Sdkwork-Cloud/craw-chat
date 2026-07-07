import { SdkworkSessionAuthBrowserRoot } from '@sdkwork/auth-pc-react';
import { getSdkworkChatIamRuntime } from '@sdkwork/im-pc-core';
/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { BrowserRouter } from 'react-router-dom';
import { AppRoutes } from './bootstrap';

export default function App() {
  return (
    <BrowserRouter>
      <SdkworkSessionAuthBrowserRoot getRuntime={getSdkworkChatIamRuntime}>
        <AppRoutes />
      </SdkworkSessionAuthBrowserRoot>
    </BrowserRouter>
  );
}
