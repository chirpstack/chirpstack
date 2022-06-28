import React, { Component } from "react";

import moment from "moment";
import { Descriptions, Space, Card, Row, Col } from "antd";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import {
  Gateway,
  GetGatewayMetricsRequest,
  GetGatewayMetricsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { Aggregation } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

import GatewayStore from "../../stores/GatewayStore";
import Map, { Marker } from "../../components/Map";
import MetricChart from "../../components/MetricChart";
import MetricHeatmap from "../../components/MetricHeatmap";
import MetricBar from "../../components/MetricBar";

interface IProps {
  gateway: Gateway;
  lastSeenAt?: Date;
}

interface IState {
  metricsAggregation: Aggregation;
  gatewayMetrics?: GetGatewayMetricsResponse;
}

class GatewayDashboard extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      metricsAggregation: Aggregation.DAY,
    };
  }

  componentDidMount() {
    this.loadMetrics();
  }

  loadMetrics = () => {
    const agg = this.state.metricsAggregation;
    const end = moment();
    let start = moment();

    if (agg === Aggregation.DAY) {
      start = start.subtract(30, "days");
    } else if (agg === Aggregation.HOUR) {
      start = start.subtract(24, "hours");
    } else if (agg === Aggregation.MONTH) {
      start = start.subtract(12, "months");
    }

    let startPb = new Timestamp();
    let endPb = new Timestamp();

    startPb.fromDate(start.toDate());
    endPb.fromDate(end.toDate());

    let req = new GetGatewayMetricsRequest();
    req.setGatewayId(this.props.gateway.getGatewayId());
    req.setStart(startPb);
    req.setEnd(endPb);
    req.setAggregation(agg);

    GatewayStore.getMetrics(req, (resp: GetGatewayMetricsResponse) => {
      this.setState({
        gatewayMetrics: resp,
      });
    });
  };

  render() {
    const loc = this.props.gateway.getLocation()!;
    const location: [number, number] = [loc.getLatitude(), loc.getLongitude()];

    if (this.state.gatewayMetrics === undefined) {
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
          <Col span={8}>
            <MetricChart
              metric={this.state.gatewayMetrics.getRxPackets()!}
              aggregation={this.state.metricsAggregation}
            />
          </Col>
          <Col span={8}>
            <MetricChart
              metric={this.state.gatewayMetrics.getTxPackets()!}
              aggregation={this.state.metricsAggregation}
            />
          </Col>
          <Col span={8}>
            <MetricHeatmap
              metric={this.state.gatewayMetrics.getRxPacketsPerFreq()!}
              aggregation={this.state.metricsAggregation}
              fromColor="rgb(227, 242, 253)"
              toColor="rgb(33, 150, 243, 1)"
            />
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={8}>
            <MetricHeatmap
              metric={this.state.gatewayMetrics.getTxPacketsPerFreq()!}
              aggregation={this.state.metricsAggregation}
              fromColor="rgb(227, 242, 253)"
              toColor="rgb(33, 150, 243, 1)"
            />
          </Col>
          <Col span={8}>
            <MetricHeatmap
              metric={this.state.gatewayMetrics.getRxPacketsPerDr()!}
              aggregation={this.state.metricsAggregation}
              fromColor="rgb(227, 242, 253)"
              toColor="rgb(33, 150, 243, 1)"
            />
          </Col>
          <Col span={8}>
            <MetricHeatmap
              metric={this.state.gatewayMetrics.getTxPacketsPerDr()!}
              aggregation={this.state.metricsAggregation}
              fromColor="rgb(227, 242, 253)"
              toColor="rgb(33, 150, 243, 1)"
            />
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={8}>
            <MetricBar
              metric={this.state.gatewayMetrics.getTxPacketsPerStatus()!}
              aggregation={this.state.metricsAggregation}
            />
          </Col>
        </Row>
      </Space>
    );
  }
}

export default GatewayDashboard;
