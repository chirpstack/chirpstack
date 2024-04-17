import { Card } from "antd";

import { TimeUnit } from "chart.js";
import { Line } from "react-chartjs-2";
import moment from "moment";
import palette from "google-palette";

import { Metric, Aggregation, MetricKind } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

interface IProps {
  metric: Metric;
  aggregation: Aggregation;
  zeroToNull?: boolean;
}

function MetricChart(props: IProps) {
  let unit: TimeUnit = "hour";
  let tooltipFormat = "LT";
  if (props.aggregation === Aggregation.DAY) {
    unit = "day";
    tooltipFormat = "MMM Do";
  } else if (props.aggregation === Aggregation.MONTH) {
    unit = "month";
    tooltipFormat = "MMM YYYY";
  } else if (props.aggregation === Aggregation.MINUTE) {
    unit = "minute";
    tooltipFormat = "LT";
  }

  const animation: false = false;

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
  let data = {
    labels: props.metric.getTimestampsList().map(v => moment(v.toDate()).valueOf()),
    datasets: props.metric
      .getDatasetsList()
      .sort((a, b) => a.getLabel().localeCompare(b.getLabel()))
      .map((v, i) => {
        const colors = palette("cb-Paired", props.metric.getDatasetsList().length).map((hex: string) => "#" + hex);
        console.log(v.getLabel());
        console.log(colors[i]);

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
                let val = v - prevValue;
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
