import { HashRouter } from "react-router-dom";
import { ImH5Shell, IM_APP_HOME_PATH } from "@sdkwork/im-h5-shell";

import { AuthGate } from "./AuthGate";

export { IM_APP_HOME_PATH };

export default function App() {
  return (
    <HashRouter>
      <AuthGate>
        <ImH5Shell />
      </AuthGate>
    </HashRouter>
  );
}
