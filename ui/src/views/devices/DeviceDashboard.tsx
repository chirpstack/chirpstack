import React, { Component } from "react";
import { Link } from "react-router-dom";

import moment from "moment";
import { Descriptions, Space, Card, Row, Col } from "antd";
import { TimeUnit } from "chart.js";
import { Line, Bar } from "react-chartjs-2";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import {
  Device,
  GetDeviceStatsRequest,
  GetDeviceStatsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { DeviceProfile } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import DeviceStore from "../../stores/DeviceStore";
import Heatmap from "../../components/Heatmap";

interface IProps {
  device: Device;
  deviceProfile: DeviceProfile;
  lastSeenAt?: Date;
}

interface IState {
  statsUp?: any;
  statsErrors?: any;
  statsUpFreq: HeatmapStats[];
  statsUpDr?: any;
  statsGwRssi?: any;
  statsGwSnr?: any;
}

interface HeatmapStats {
  x: string;
  y: Array<[string, number]>;
}

class DeviceDashboard extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      statsUpFreq: [],
    };
  }

  componentDidMount() {
    this.loadStats();
  }

  loadStats = () => {
    const end = moment().toDate();
    const start = moment().subtract(30, "days").toDate();

    let startPb = new Timestamp();
    let endPb = new Timestamp();

    startPb.fromDate(start);
    endPb.fromDate(end);

    let req = new GetDeviceStatsRequest();
    req.setDevEui(this.props.device.getDevEui());
    req.setStart(startPb);
    req.setEnd(endPb);

    DeviceStore.getStats(req, (resp: GetDeviceStatsResponse) => {
      let statsUp: {
        labels: string[];
        datasets: {
          label: string;
          borderColor: string;
          backgroundColor: string;
          lineTension: number;
          pointBackgroundColor: string;
          data: number[];
        }[];
      } = {
        labels: [],
        datasets: [
          {
            label: "uplink",
            borderColor: "rgba(33, 150, 243, 1)",
            backgroundColor: "rgba(0, 0, 0, 0)",
            lineTension: 0,
            pointBackgroundColor: "rgba(33, 150, 243, 1)",
            data: [],
          },
        ],
      };

      let statsErrors: {
        labels: string[];
        datasets: {
          label: string;
          data: number[];
          backgroundColor: string;
        }[];
      } = {
        labels: [],
        datasets: [],
      };

      let statsErrorsSet: {
        [key: string]: number[];
      } = {};

      let statsUpDr: {
        labels: string[];
        datasets: {
          label: string;
          data: number[];
          backgroundColor: string;
        }[];
      } = {
        labels: [],
        datasets: [],
      };

      let statsUpDrSet: {
        [key: string]: number[];
      } = {};

      let statsGwRssiLabels: string[] = [];
      let statsGwRssiData: (number | null)[] = [];
      let statsGwRssi = {
        labels: statsGwRssiLabels,
        datasets: [
          {
            label: "rssi (reported by gateways)",
            borderColor: "rgba(33, 150, 243, 1)",
            backgroundColor: "rgba(0, 0, 0, 0)",
            lineTension: 0,
            pointBackgroundColor: "rgba(33, 150, 243, 1)",
            data: statsGwRssiData,
          },
        ],
      };

      let statsGwSnrLabels: string[] = [];
      let statsGwSnrData: (number | null)[] = [];
      let statsGwSnr = {
        labels: statsGwSnrLabels,
        datasets: [
          {
            label: "rssi (reported by gateways)",
            borderColor: "rgba(33, 150, 243, 1)",
            backgroundColor: "rgba(0, 0, 0, 0)",
            lineTension: 0,
            pointBackgroundColor: "rgba(33, 150, 243, 1)",
            data: statsGwSnrData,
          },
        ],
      };

      let statsUpFreq: HeatmapStats[] = [];
      for (const row of resp.getResultList()) {
        statsUp.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));
        statsUp.datasets[0].data.push(row.getRxPackets());

        statsUpFreq.push({
          x: moment(row.getTime()!.toDate()).format("YYYY-MM-DD"),
          y: row
            .getRxPacketsPerFrequencyMap()
            .toObject()
            .map(v => [v[0].toString(), v[1]]),
        });

        statsErrors.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));
        statsUpDr.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));
        statsGwRssi.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));
        statsGwSnr.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));

        if (row.getRxPackets() !== 0) {
          statsGwRssi.datasets[0].data.push(row.getGwRssi());
          statsGwSnr.datasets[0].data.push(row.getGwSnr());
        } else {
          statsGwRssi.datasets[0].data.push(null);
          statsGwSnr.datasets[0].data.push(null);
        }

        for (const v of row.getErrorsMap().toObject()) {
          if (statsErrorsSet[v[0]] === undefined) {
            statsErrorsSet[v[0]] = [];
          }

          // fill gaps with 0s
          for (let i = statsErrorsSet[v[0]].length; i < statsErrors.labels.length - 1; i++) {
            statsErrorsSet[v[0]].push(0);
          }

          statsErrorsSet[v[0]].push(v[1]);
        }

        for (const v of row.getRxPacketsPerDrMap().toObject()) {
          if (statsUpDrSet[v[0]] === undefined) {
            statsUpDrSet[v[0]] = [];
          }

          // fill gaps with 0s
          for (let i = statsUpDrSet[v[0]].length; i < statsUpDr.labels.length - 1; i++) {
            statsUpDrSet[v[0]].push(0);
          }

          statsUpDrSet[v[0]].push(v[1]);
        }
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
      Object.entries(statsErrorsSet).forEach(([k, v]) => {
        statsErrors.datasets.push({
          label: k,
          data: v,
          backgroundColor: backgroundColors.shift()!,
        });
      });

      backgroundColors = [
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
      Object.entries(statsUpDrSet).forEach(([k, v]) => {
        statsUpDr.datasets.push({
          label: k,
          data: v,
          backgroundColor: backgroundColors.shift()!,
        });
      });

      this.setState({
        statsUp: statsUp,
        statsErrors: statsErrors,
        statsUpFreq: statsUpFreq,
        statsUpDr: statsUpDr,
        statsGwRssi: statsGwRssi,
        statsGwSnr: statsGwSnr,
      });
    });
  };

  render() {
    const animation: false = false;
    const unit: TimeUnit = "day";

    const barOptions = {
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
          time: {
            unit: unit,
          },
        },
      },
    };

    const statsOptions = {
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
          time: {
            unit: unit,
          },
        },
      },
    };

    if (this.state.statsUpDr === undefined) {
      return null;
    }

    let lastSeenAt = "Never";
    if (this.props.lastSeenAt !== undefined) {
      lastSeenAt = moment(this.props.lastSeenAt).format("YYYY-MM-DD HH:mm:ss");
    }

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <Card>
          <Descriptions>
            <Descriptions.Item label="Last seen">{lastSeenAt}</Descriptions.Item>
            <Descriptions.Item label="Device profile">
              <Link
                to={`/tenants/${this.props.deviceProfile.getTenantId()}/device-profiles/${this.props.deviceProfile.getId()}/edit`}
              >
                {this.props.deviceProfile.getName()}
              </Link>
            </Descriptions.Item>
            <Descriptions.Item label="Enabled">{this.props.device.getIsDisabled() ? "no" : "yes"}</Descriptions.Item>
            <Descriptions.Item label="Description">{this.props.device.getDescription()}</Descriptions.Item>
          </Descriptions>
        </Card>
        <Row gutter={24}>
          <Col span={12}>
            <Card title="Received" className="dashboard-chart">
              <Line height={75} options={statsOptions} data={this.state.statsUp} />
            </Card>
          </Col>
          <Col span={12}>
            <Card title="Errors" className="dashboard-chart">
              <Bar data={this.state.statsErrors} options={barOptions} />
            </Card>
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={12}>
            <Card title="SNR" className="dashboard-chart">
              <Line height={75} options={statsOptions} data={this.state.statsGwSnr} />
            </Card>
          </Col>
          <Col span={12}>
            <Card title="RSSI" className="dashboard-chart">
              <Line height={75} options={statsOptions} data={this.state.statsGwRssi} />
            </Card>
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={12}>
            <Card title="Received / frequency" className="dashboard-chart">
              <Heatmap data={this.state.statsUpFreq} fromColor="rgb(227, 242, 253)" toColor="rgb(33, 150, 243, 1)" />
            </Card>
          </Col>
          <Col span={12}>
            <Card title="Received / DR" className="dashboard-chart">
              <Bar data={this.state.statsUpDr} options={barOptions} />
            </Card>
          </Col>
        </Row>
      </Space>
    );
  }
}

export default DeviceDashboard;
