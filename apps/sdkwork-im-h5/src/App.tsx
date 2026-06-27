import { HashRouter, Navigate, useLocation } from "react-router-dom";

import { AppAuthGate } from "./AppAuthGate";
import { ImApp } from "./ImApp";
import { IM_APP_HOME_PATH } from "./constants/appRoutes";
import { bootstrap } from "./bootstrap/runtime";

import "@sdkwork/im-h5-shell/styles.css";

bootstrap();

function AppShell() {
  const location = useLocation();
  const route = location.pathname;

  if (route === "/" || route === "") {
    return <Navigate replace to={IM_APP_HOME_PATH} />;
  }

  return (
    <AppAuthGate>
      <ImApp route={route} />
    </AppAuthGate>
  );
}

export default function App() {
  return (
    <HashRouter>
      <AppShell />
    </HashRouter>
  );
}
