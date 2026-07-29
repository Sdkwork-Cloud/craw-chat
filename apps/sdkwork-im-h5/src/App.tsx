import { HashRouter } from "react-router-dom";

import { AuthGate } from "./AuthGate";
import { ImApp, IM_APP_HOME_PATH } from "./ImApp";

export { IM_APP_HOME_PATH };

export default function App() {
  return (
    <HashRouter>
      <AuthGate>
        <ImApp />
      </AuthGate>
    </HashRouter>
  );
}
