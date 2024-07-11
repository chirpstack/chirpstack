import React from 'react'
import ReactDOM from 'react-dom/client'

import { Chart, registerables } from "chart.js";
import { MatrixElement, MatrixController } from "chartjs-chart-matrix";
import "chartjs-adapter-moment";

import App from './App.tsx'
import reportWebVitals from "./reportWebVitals.ts";

import "antd/dist/reset.css";
import "leaflet/dist/leaflet.css";
import "leaflet.awesome-markers/dist/leaflet.awesome-markers.css";
import "@fortawesome/fontawesome-free/css/all.css";
import "codemirror/lib/codemirror.css";
import "codemirror/theme/base16-light.css";
import "./index.css";

Chart.register(MatrixController, MatrixElement, ...registerables);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)

reportWebVitals();
