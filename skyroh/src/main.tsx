import React from "react";
import ReactDOM from "react-dom/client";

import "@/styles/globals.css"
import Home from "@/app/page";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Home />
  </React.StrictMode>,
);
