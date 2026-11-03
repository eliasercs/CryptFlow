import React from "react";
import ReactDOM from "react-dom/client";

import { ServicesProvider, type Services } from "./application/services";
import { TauriAuthService } from "./infrastructure/tauri/TauriAuthService";
import { App } from "./ui/App";
import "./ui/styles.css";

// Único lugar que conoce las implementaciones concretas.
const services: Services = {
  auth: new TauriAuthService(),
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ServicesProvider services={services}>
      <App />
    </ServicesProvider>
  </React.StrictMode>,
);