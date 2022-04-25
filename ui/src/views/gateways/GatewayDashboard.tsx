import React, { Component } from "react";

import moment from "moment";
import { Descriptions, Space, Card, Row, Col } from "antd";
import { TimeUnit } from "chart.js";
import { Line, Bar } from "react-chartjs-2";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import {
  Gateway,
  GetGatewayStatsRequest,
  GetGatewayStatsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import GatewayStore from "../../stores/GatewayStore";
import Map, { Marker } from "../../components/Map";
import Heatmap from "../../components/Heatmap";

interface HeatmapStats {
  x: string;
  y: Array<[string, number]>;
}

interface IProps {
  gateway: Gateway;
  lastSeenAt?: Date;
}

interface IState {
  statsUp?: any;
  statsDown?: any;
  statsUpFreq: HeatmapStats[];
  statsDownFreq: HeatmapStats[];
  statsUpDr: HeatmapStats[];
  statsDownDr: HeatmapStats[];
  statsDownStatus?: any;
}

class GatewayDashboard extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      statsUpFreq: [],
      statsDownFreq: [],
      statsUpDr: [],
      statsDownDr: [],
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

    let req = new GetGatewayStatsRequest();
    req.setGatewayId(this.props.gateway.getGatewayId());
    req.setStart(startPb);
    req.setEnd(endPb);

    GatewayStore.getStats(req, (resp: GetGatewayStatsResponse) => {
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
            label: "rx received",
            borderColor: "rgba(33, 150, 243, 1)",
            backgroundColor: "rgba(0, 0, 0, 0)",
            lineTension: 0,
            pointBackgroundColor: "rgba(33, 150, 243, 1)",
            data: [],
          },
        ],
      };

      let statsDown: {
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
            label: "rx received",
            borderColor: "rgba(33, 150, 243, 1)",
            backgroundColor: "rgba(0, 0, 0, 0)",
            lineTension: 0,
            pointBackgroundColor: "rgba(33, 150, 243, 1)",
            data: [],
          },
        ],
      };

      let statsDownStatus: {
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

      let statsDownStatusSet: {
        [key: string]: number[];
      } = {};

      let statsUpFreq: HeatmapStats[] = [];
      let statsDownFreq: HeatmapStats[] = [];
      let statsUpDr: HeatmapStats[] = [];
      let statsDownDr: HeatmapStats[] = [];

      for (const row of resp.getResultList()) {
        statsUp.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));
        statsDown.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));
        statsDownStatus.labels.push(moment(row.getTime()!.toDate()).format("YYYY-MM-DD"));

        statsUp.datasets[0].data.push(row.getRxPackets());
        statsDown.datasets[0].data.push(row.getTxPackets());

        statsUpFreq.push({
          x: moment(row.getTime()!.toDate()).format("YYYY-MM-DD"),
          y: row
            .getRxPacketsPerFrequencyMap()
            .toObject()
            .map(v => [v[0].toString(), v[1]]),
        });

        statsDownFreq.push({
          x: moment(row.getTime()!.toDate()).format("YYYY-MM-DD"),
          y: row
            .getTxPacketsPerFrequencyMap()
            .toObject()
            .map(v => [v[0].toString(), v[1]]),
        });

        statsUpDr.push({
          x: moment(row.getTime()!.toDate()).format("YYYY-MM-DD"),
          y: row
            .getRxPacketsPerDrMap()
            .toObject()
            .map(v => [v[0].toString(), v[1]]),
        });

        statsDownDr.push({
          x: moment(row.getTime()!.toDate()).format("YYYY-MM-DD"),
          y: row
            .getTxPacketsPerDrMap()
            .toObject()
            .map(v => [v[0].toString(), v[1]]),
        });

        for (const v of row.getTxPacketsPerStatusMap().toObject()) {
          if (statsDownStatusSet[v[0]] === undefined) {
            statsDownStatusSet[v[0]] = [];
          }
          // fill gaps with 0s
          for (let i = statsDownStatusSet[v[0]].length; i < statsDownStatus.labels.length - 1; i++) {
            statsDownStatusSet[v[0]].push(0);
          }
          statsDownStatusSet[v[0]].push(v[1]);
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
      Object.entries(statsDownStatusSet).forEach(([k, v]) => {
        statsDownStatus.datasets.push({
          label: k,
          data: v,
          backgroundColor: backgroundColors.shift()!,
        });
      });

      this.setState({
        statsUp: statsUp,
        statsDown: statsDown,
        statsUpFreq: statsUpFreq,
        statsDownFreq: statsDownFreq,
        statsUpDr: statsUpDr,
        statsDownDr: statsDownDr,
        statsDownStatus: statsDownStatus,
      });
    });
  };

  render() {
    const loc = this.props.gateway.getLocation()!;
    const location: [number, number] = [loc.getLatitude(), loc.getLongitude()];

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

    if (this.state.statsUp === undefined) {
      return null;
    }

    let lastSeenAt: string = "Never";
    if (this.props.lastSeenAt !== undefined) {
      lastSeenAt = moment(this.props.lastSeenAt).format("YYYY-MM-DD HH:mm:ss");
    }

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <Card>
          <Descriptions>
            <Descriptions.Item label="Last seen">{lastSeenAt}</Descriptions.Item>
            <Descriptions.Item label="Region">
              {this.props.gateway.getPropertiesMap().get("region_name")}
            </Descriptions.Item>
            <Descriptions.Item label="Region common-name">
              {this.props.gateway.getPropertiesMap().get("region_common_name")}
            </Descriptions.Item>
            <Descriptions.Item label="Description">{this.props.gateway.getDescription()}</Descriptions.Item>
          </Descriptions>
        </Card>
        <Row gutter={24}>
          <Col span={24}>
            <Map height={500} center={location}>
              <Marker position={location} faIcon="wifi" color="blue" />
            </Map>
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={12}>
            <Card title="Received" className="dashboard-chart">
              <Line height={75} options={statsOptions} data={this.state.statsUp} />
            </Card>
          </Col>
          <Col span={12}>
            <Card title="Transmitted" className="dashboard-chart">
              <Line height={75} options={statsOptions} data={this.state.statsDown} />
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
            <Card title="Transmitted / frequency" className="dashboard-chart">
              <Heatmap data={this.state.statsDownFreq} fromColor="rgb(227, 242, 253)" toColor="rgb(33, 150, 243, 1)" />
            </Card>
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={12}>
            <Card title="Received / DR" className="dashboard-chart">
              <Heatmap data={this.state.statsUpDr} fromColor="rgb(227, 242, 253)" toColor="rgb(33, 150, 243, 1)" />
            </Card>
          </Col>
          <Col span={12}>
            <Card title="Transmitted / DR" className="dashboard-chart">
              <Heatmap data={this.state.statsDownDr} fromColor="rgb(227, 242, 253)" toColor="rgb(33, 150, 243, 1)" />
            </Card>
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={12}>
            <Card title="Transmission / Ack status" className="dashboard-chart">
              <Bar data={this.state.statsDownStatus} options={barOptions} />
            </Card>
          </Col>
        </Row>
      </Space>
    );
  }
}

export default GatewayDashboard;
