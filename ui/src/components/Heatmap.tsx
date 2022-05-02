import React, { Component } from "react";

import { color } from "chart.js/helpers";
import { Chart } from "react-chartjs-2";

interface HeatmapData {
  x: string;
  y: Array<[string, number]>;
}

interface IProps {
  data: HeatmapData[];
  fromColor: string;
  toColor: string;
}

class Heatmap extends Component<IProps> {
  render() {
    if (this.props.data.length === 0) {
      return null;
    }

    let xSet: { [key: string]: any } = {};
    let ySet: { [key: string]: any } = {};

    let dataData: {
      x: string;
      y: string;
      v: number;
    }[] = [];

    let data = {
      labels: [],
      datasets: [
        {
          label: "Heatmap",
          data: dataData,
          minValue: -1,
          maxValue: -1,
          xSet: xSet,
          ySet: ySet,
          fromColor: this.props.fromColor.match(/\d+/g)!.map(Number),
          toColor: this.props.toColor.match(/\d+/g)!.map(Number),
          backgroundColor: (ctx: any): string => {
            if (
              ctx.dataset === undefined ||
              ctx.dataset.data === undefined ||
              ctx.dataset.data[ctx.dataIndex] === undefined
            ) {
              return color("white").rgbString();
            }

            const value = ctx.dataset.data[ctx.dataIndex].v;
            const steps = ctx.dataset.maxValue - ctx.dataset.minValue + 1;
            const step = value - ctx.dataset.minValue;
            const factor = (1 / steps) * step;

            let result: [number, number, number] = ctx.dataset.fromColor.slice();
            for (var i = 0; i < 3; i++) {
              result[i] = Math.round(result[i] + factor * (ctx.dataset.toColor[i] - ctx.dataset.fromColor[i]));
            }

            return color(result).rgbString();
          },
          borderWidth: 0,
          width: (ctx: any) => {
            return (ctx.chart.chartArea || {}).width / Object.keys(ctx.dataset.xSet).length - 1;
          },
          height: (ctx: any) => {
            return (ctx.chart.chartArea || {}).height / Object.keys(ctx.dataset.ySet).length - 1;
          },
        },
      ],
    };

    let xLabels: string[] = [];

    const animation: false = false;

    let options = {
      animation: animation,
      maintainAspectRatio: false,
      scales: {
        y: {
          type: "category" as const,
          offset: true,
          grid: {
            display: false,
          },
        },
        x: {
          type: "time" as const,
          offset: true,
          labels: xLabels,
          grid: {
            display: false,
          },
        },
      },
      plugins: {
        legend: { display: false },
        tooltip: {
          callbacks: {
            title: () => {
              return "";
            },
            label: (ctx: any) => {
              const v = ctx.dataset.data[ctx.dataIndex].v;
              return "Count: " + v;
            },
          },
        },
      },
    };

    for (const row of this.props.data) {
      options.scales.x.labels.push(row.x);
      data.datasets[0].xSet[row.x] = {};

      for (const y of row.y) {
        data.datasets[0].ySet[y[0]] = {};

        data.datasets[0].data.push({
          x: row.x,
          y: y[0],
          v: y[1],
        });

        if (data.datasets[0].minValue === -1 || data.datasets[0].minValue > y[1]) {
          data.datasets[0].minValue = y[1];
        }

        if (data.datasets[0].maxValue < y[1]) {
          data.datasets[0].maxValue = y[1];
        }
      }
    }

    return <Chart type="matrix" data={data} options={options} />;
  }
}

export default Heatmap;
