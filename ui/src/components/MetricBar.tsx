import { Card } from "antd";

import { TimeUnit } from "chart.js";
import { Bar } from "react-chartjs-2";
import moment from "moment";
import palette from "google-palette";

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

  props.metric.getDatasetsList().forEach((ds, i) => {
    data.datasets.push({
      label: ds.getLabel(),
      data: ds.getDataList(),
      backgroundColor: palette("cb-Paired", props.metric.getDatasetsList().length).map((hex: string) => "#" + hex)[i],
    });
  });

  return (
    <Card title={props.metric.getName()} className="dashboard-chart">
      <Bar data={data} options={options} />
    </Card>
  );
}

export default MetricBar;
