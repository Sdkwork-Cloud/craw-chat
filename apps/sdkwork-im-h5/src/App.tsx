import { BrowserRouter } from "react-router-dom";
import { ImH5Shell, IM_APP_HOME_PATH } from "@sdkwork/im-h5-shell";

import { AuthGate } from "./AuthGate";
import { resolveConfiguredImH5ModuleIds } from "./bootstrap/composition";

export { IM_APP_HOME_PATH };

export default function App() {
  const moduleIds = resolveConfiguredImH5ModuleIds();

  return (
    <BrowserRouter>
      <AuthGate>
        <ImH5Shell moduleIds={moduleIds} />
      </AuthGate>
    </BrowserRouter>
  );
}
