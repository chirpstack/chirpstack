import { Card } from "antd";

import type { TimeUnit } from "chart.js";
import { Line } from "react-chartjs-2";
import palette from "google-palette";

import type { Metric } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { Aggregation, MetricKind } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

interface IProps {
  metric: Metric;
  aggregation: Aggregation;
  zeroToNull?: boolean;
}

function MetricChart(props: IProps) {
  let unit: TimeUnit = "hour";
  let tooltipFormat = "p";
  if (props.aggregation === Aggregation.DAY) {
    unit = "day";
    tooltipFormat = "MMM d";
  } else if (props.aggregation === Aggregation.MONTH) {
    unit = "month";
    tooltipFormat = "MMM yyyy";
  } else if (props.aggregation === Aggregation.MINUTE) {
    unit = "minute";
    tooltipFormat = "p";
  }

  const animation = false as const;

  const options = {
    animation: animation,
    plugins: {
      legend: {
        display: false,
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
          tooltipFormat: tooltipFormat,
        },
      },
    },
  };

  let prevValue = 0;
  const data = {
    labels: props.metric.getTimestampsList().map(v => v.toDate().getTime()),
    datasets: props.metric
      .getDatasetsList()
      .sort((a, b) => a.getLabel().localeCompare(b.getLabel()))
      .map((v, i) => {
        const colors = palette("cb-Paired", props.metric.getDatasetsList().length).map((hex: string) => "#" + hex);

        return {
          label: v.getLabel(),
          borderColor: colors[i],
          pointBackgroundColor: colors[i],
          lineTension: 0,
          data: v.getDataList().map(v => {
            if (v === 0 && props.zeroToNull) {
              return null;
            } else {
              if (props.metric.getKind() === MetricKind.COUNTER) {
                const val = v - prevValue;
                prevValue = v;
                if (val < 0) {
                  return 0;
                }
                return val;
              } else {
                return v;
              }
            }
          }),
        };
      }),
  };

  let name = props.metric.getName();
  if (props.metric.getKind() === MetricKind.COUNTER) {
    name = `${name} (per ${unit})`;
  }

  return (
    <Card title={name} className="dashboard-chart">
      <Line height={75} options={options} data={data} />
    </Card>
  );
}

export default MetricChart;
