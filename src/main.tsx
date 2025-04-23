import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
// initialize i18n before rendering the app
import "./libs/i18n";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
