import React, { Component } from "react";

import { Card } from "antd";

import { TimeUnit } from "chart.js";
import { Line } from "react-chartjs-2";
import moment from "moment";

import { Metric, Aggregation } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

interface IProps {
  metric: Metric;
  aggregation: Aggregation;
  zeroToNull?: boolean;
}

class MetricChart extends Component<IProps> {
  render() {
    let unit: TimeUnit = "hour";
    if (this.props.aggregation === Aggregation.DAY) {
      unit = "day";
    } else if (this.props.aggregation === Aggregation.MONTH) {
      unit = "month";
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
          },
        },
      },
    };

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
              return v;
            }
          }),
        };
      }),
    };

    return (
      <Card title={this.props.metric.getName()} className="dashboard-chart">
        <Line height={75} options={options} data={data} />
      </Card>
    );
  }
}

export default MetricChart;
