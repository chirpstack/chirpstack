import React from "react";
import ReactDOM from "react-dom/client";

import { Chart, registerables } from "chart.js";
import { MatrixElement, MatrixController } from "chartjs-chart-matrix";
import "chartjs-adapter-date-fns";

import App from "./App";

import "antd/dist/reset.css";
import "leaflet/dist/leaflet.css";
import "leaflet.awesome-markers/dist/leaflet.awesome-markers.css";
import "@fortawesome/fontawesome-free/css/all.css";
import "./index.css";

Chart.register(MatrixController, MatrixElement, ...registerables);

const root = ReactDOM.createRoot(document.getElementById("root") as HTMLElement);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
