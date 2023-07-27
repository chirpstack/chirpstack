import { Card } from "antd";

import { TimeUnit } from "chart.js";
import { Bar } from "react-chartjs-2";
import moment from "moment";

import { Metric, Aggregation } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

interface IProps {
  metric: Metric;
  aggregation: Aggregation;
}

function MetricBar(props: IProps) {
  let unit: TimeUnit = "hour";
  if (props.aggregation === Aggregation.DAY) {
    unit = "day";
  } else if (props.aggregation === Aggregation.MONTH) {
    unit = "month";
  }

  let backgroundColors = [
    "#8bc34a",
    "#ff5722",
    "#ff9800",
    "#ffc107",
    "#ffeb3b",
    "#cddc39",
    "#4caf50",
    "#009688",
    "#00bcd4",
    "#03a9f4",
    "#2196f3",
    "#3f51b5",
    "#673ab7",
    "#9c27b0",
    "#e91e63",
  ];

  const animation: false = false;

  const options = {
    animation: animation,
    plugins: {
      legend: {
        display: true,
      },
    },
    maintainAspectRatio: false,
    scales: {
      y: {
        beginAtZero: true,
      },
      x: {
        type: "time" as const,
        time: {
          unit: unit,
        },
      },
    },
  };

  let data: {
    labels: number[];
    datasets: {
      label: string;
      data: number[];
      backgroundColor: string;
    }[];
  } = {
    labels: props.metric.getTimestampsList().map(v => moment(v.toDate()).valueOf()),
    datasets: [],
  };

  for (let ds of props.metric.getDatasetsList()) {
    data.datasets.push({
      label: ds.getLabel(),
      data: ds.getDataList(),
      backgroundColor: backgroundColors.shift()!,
    });
  }

  return (
    <Card title={props.metric.getName()} className="dashboard-chart">
      <Bar data={data} options={options} />
    </Card>
  );
}

export default MetricBar;
