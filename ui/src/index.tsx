import React from "react";
import ReactDOM from "react-dom/client";

import { Chart, registerables } from "chart.js";
import { MatrixElement, MatrixController } from "chartjs-chart-matrix";
import "chartjs-adapter-moment";

import App from "./App";
import reportWebVitals from "./reportWebVitals";

import "antd/dist/reset.css";
import "leaflet/dist/leaflet.css";
import "leaflet.awesome-markers/dist/leaflet.awesome-markers.css";
import "@fortawesome/fontawesome-free/css/all.css";
import "codemirror/lib/codemirror.css";
import "codemirror/theme/base16-light.css";
import "./index.css";

Chart.register(MatrixController, MatrixElement, ...registerables);

const root = ReactDOM.createRoot(document.getElementById("root") as HTMLElement);
root.render(<App />);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
