import React, { Component } from "react";

import { Card } from "antd";

import { TimeUnit } from "chart.js";
import { Line } from "react-chartjs-2";
import moment from "moment";

import { Metric, Aggregation, MetricKind } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

interface IProps {
  metric: Metric;
  aggregation: Aggregation;
  zeroToNull?: boolean;
}

class MetricChart extends Component<IProps> {
  render() {
    let unit: TimeUnit = "hour";
    let tooltipFormat = "LT";
    if (this.props.aggregation === Aggregation.DAY) {
      unit = "day";
      tooltipFormat = "MMM Do";
    } else if (this.props.aggregation === Aggregation.MONTH) {
      unit = "month";
      tooltipFormat = "MMM YYYY";
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
      labels: this.props.metric.getTimestampsList().map(v => moment(v.toDate()).valueOf()),
      datasets: this.props.metric.getDatasetsList().map(v => {
        return {
          label: v.getLabel(),
          borderColor: "rgba(33, 150, 243, 1)",
          backgroundColor: "rgba(0, 0, 0, 0)",
          lineTension: 0,
          pointBackgroundColor: "rgba(33, 150, 243, 1)",
          data: v.getDataList().map(v => {
            if (v === 0 && this.props.zeroToNull) {
              return null;
            } else {
              if (this.props.metric.getKind() === MetricKind.COUNTER) {
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

    let name = this.props.metric.getName();
    if (this.props.metric.getKind() === MetricKind.COUNTER) {
      name = `${name} (per ${unit})`;
    }

    return (
      <Card title={name} className="dashboard-chart">
        <Line height={75} options={options} data={data} />
      </Card>
    );
  }
}

export default MetricChart;
