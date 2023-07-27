import { Card } from "antd";

import { color } from "chart.js/helpers";
import { TimeUnit } from "chart.js";
import { Chart } from "react-chartjs-2";
import moment from "moment";

import { Metric, Aggregation } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

interface IProps {
  metric: Metric;
  fromColor: string;
  toColor: string;
  aggregation: Aggregation;
}

function MetricHeatmap(props: IProps) {
  let unit: TimeUnit = "hour";
  if (props.aggregation === Aggregation.DAY) {
    unit = "day";
  } else if (props.aggregation === Aggregation.MONTH) {
    unit = "month";
  }

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
        time: {
          unit: unit,
        },
        offset: true,
        labels: props.metric.getTimestampsList().map(v => moment(v.toDate().valueOf())),
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

  let dataData: {
    x: number;
    y: string;
    v: number;
  }[] = [];

  let data = {
    labels: props.metric.getDatasetsList().map(v => v.getLabel()),
    datasets: [
      {
        label: "Heatmap",
        data: dataData,
        minValue: -1,
        maxValue: -1,
        fromColor: props.fromColor.match(/\d+/g)!.map(Number),
        toColor: props.toColor.match(/\d+/g)!.map(Number),
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
          return (ctx.chart.chartArea || {}).width / props.metric.getTimestampsList().length - 1;
        },
        height: (ctx: any) => {
          return (ctx.chart.chartArea || {}).height / props.metric.getDatasetsList().length - 1;
        },
      },
    ],
  };

  data.labels.sort();

  const tsList = props.metric.getTimestampsList();
  const dsList = props.metric.getDatasetsList();

  for (let i = 0; i < tsList.length; i++) {
    for (let ds of dsList) {
      let v = ds.getDataList()[i];
      if (v === 0) {
        continue;
      }

      data.datasets[0].data.push({
        x: moment(tsList[i].toDate()).valueOf(),
        y: ds.getLabel(),
        v: v,
      });

      if (data.datasets[0].minValue === -1 || data.datasets[0].minValue > v) {
        data.datasets[0].minValue = v;
      }

      if (data.datasets[0].maxValue < v) {
        data.datasets[0].maxValue = v;
      }
    }
  }

  return (
    <Card title={props.metric.getName()} className="dashboard-chart">
      <Chart type="matrix" data={data} options={options} />
    </Card>
  );
}

export default MetricHeatmap;
