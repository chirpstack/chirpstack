import React, { Component } from "react";
import { Link } from "react-router-dom";

import moment from "moment";
import { ReloadOutlined } from "@ant-design/icons";
import { Descriptions, Space, Card, Statistic, Row, Col, Tabs, Radio, RadioChangeEvent, Button, Spin } from "antd";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import {
  Device,
  GetDeviceMetricsRequest,
  GetDeviceMetricsResponse,
  GetDeviceLinkMetricsRequest,
  GetDeviceLinkMetricsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { Aggregation } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { DeviceProfile } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import DeviceStore from "../../stores/DeviceStore";
import MetricChart from "../../components/MetricChart";
import MetricHeatmap from "../../components/MetricHeatmap";
import MetricBar from "../../components/MetricBar";

interface IProps {
  device: Device;
  deviceProfile: DeviceProfile;
  lastSeenAt?: Date;
}

interface IState {
  metricsAggregation: Aggregation;
  deviceMetrics?: GetDeviceMetricsResponse;
  deviceLinkMetrics?: GetDeviceLinkMetricsResponse;
  deviceMetricsLoaded: boolean;
  deviceLinkMetricsLoaded: boolean;
}

class DeviceDashboard extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      metricsAggregation: Aggregation.DAY,
      deviceMetricsLoaded: false,
      deviceLinkMetricsLoaded: false,
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

    this.setState(
      {
        deviceMetricsLoaded: false,
        deviceLinkMetricsLoaded: false,
      },
      () => {
        this.loadLinkMetrics(start.toDate(), end.toDate(), agg);
        this.loadDeviceMetrics(start.toDate(), end.toDate(), agg);
      },
    );
  };

  loadDeviceMetrics = (start: Date, end: Date, agg: Aggregation) => {
    let startPb = new Timestamp();
    let endPb = new Timestamp();

    startPb.fromDate(start);
    endPb.fromDate(end);

    let req = new GetDeviceMetricsRequest();
    req.setDevEui(this.props.device.getDevEui());
    req.setStart(startPb);
    req.setEnd(endPb);
    req.setAggregation(agg);

    DeviceStore.getMetrics(req, (resp: GetDeviceMetricsResponse) => {
      this.setState({
        deviceMetrics: resp,
        deviceMetricsLoaded: true,
      });
    });
  };

  loadLinkMetrics = (start: Date, end: Date, agg: Aggregation) => {
    let startPb = new Timestamp();
    let endPb = new Timestamp();

    startPb.fromDate(start);
    endPb.fromDate(end);

    let req = new GetDeviceLinkMetricsRequest();
    req.setDevEui(this.props.device.getDevEui());
    req.setStart(startPb);
    req.setEnd(endPb);
    req.setAggregation(agg);

    DeviceStore.getLinkMetrics(req, (resp: GetDeviceLinkMetricsResponse) => {
      this.setState({
        deviceLinkMetrics: resp,
        deviceLinkMetricsLoaded: true,
      });
    });
  };

  onMetricsAggregationChange = (e: RadioChangeEvent) => {
    this.setState(
      {
        metricsAggregation: e.target.value,
      },
      this.loadMetrics,
    );
  };

  render() {
    if (this.state.deviceLinkMetrics === undefined || this.state.deviceMetrics === undefined) {
      return null;
    }

    let deviceMetrics = [];

    {
      let states = this.state.deviceMetrics.getStatesMap();
      let keys = states.toArray().map(v => v[0]);
      keys.sort();

      for (let i = 0; i < keys.length; i += 3) {
        let items = keys.slice(i, i + 3).map(k => {
          let m = states.get(k)!;
          return (
            <Col span={8}>
              <Card>
                <Statistic title={m.getName()} value={m.getValue()} />
              </Card>
            </Col>
          );
        });

        deviceMetrics.push(<Row gutter={24}>{items}</Row>);
      }
    }

    {
      let metrics = this.state.deviceMetrics.getMetricsMap();
      let keys = metrics.toArray().map(v => v[0]);
      keys.sort();

      for (let i = 0; i < keys.length; i += 3) {
        let items = keys.slice(i, i + 3).map(k => {
          let m = metrics.get(k)!;
          return (
            <Col span={8}>
              <MetricChart metric={m} aggregation={this.state.metricsAggregation} zeroToNull />
            </Col>
          );
        });

        deviceMetrics.push(<Row gutter={24}>{items}</Row>);
      }
    }

    let lastSeenAt = "Never";
    if (this.props.lastSeenAt !== undefined) {
      lastSeenAt = moment(this.props.lastSeenAt).format("YYYY-MM-DD HH:mm:ss");
    }

    const loading = !this.state.deviceLinkMetricsLoaded || !this.state.deviceMetrics;

    const aggregations = (
      <Space direction="horizontal">
        {loading && <Spin size="small" />}
        <Radio.Group value={this.state.metricsAggregation} onChange={this.onMetricsAggregationChange} size="small">
          <Radio.Button value={Aggregation.HOUR} disabled={loading}>
            24h
          </Radio.Button>
          <Radio.Button value={Aggregation.DAY} disabled={loading}>
            31d
          </Radio.Button>
          <Radio.Button value={Aggregation.MONTH} disabled={loading}>
            1y
          </Radio.Button>
        </Radio.Group>
        <Button type="primary" size="small" icon={<ReloadOutlined />} onClick={this.loadMetrics} disabled={loading} />
      </Space>
    );

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
        <Tabs tabBarExtraContent={aggregations}>
          <Tabs.TabPane tab="Link metrics" key="1">
            <Space direction="vertical" style={{ width: "100%" }} size="large">
              <Row gutter={24}>
                <Col span={8}>
                  <MetricChart
                    metric={this.state.deviceLinkMetrics.getRxPackets()!}
                    aggregation={this.state.metricsAggregation}
                  />
                </Col>
                <Col span={8}>
                  <MetricChart
                    metric={this.state.deviceLinkMetrics.getGwRssi()!}
                    aggregation={this.state.metricsAggregation}
                    zeroToNull
                  />
                </Col>
                <Col span={8}>
                  <MetricChart
                    metric={this.state.deviceLinkMetrics.getGwSnr()!}
                    aggregation={this.state.metricsAggregation}
                    zeroToNull
                  />
                </Col>
              </Row>
              <Row gutter={24}>
                <Col span={8}>
                  <MetricHeatmap
                    metric={this.state.deviceLinkMetrics.getRxPacketsPerFreq()!}
                    aggregation={this.state.metricsAggregation}
                    fromColor="rgb(227, 242, 253)"
                    toColor="rgb(33, 150, 243, 1)"
                  />
                </Col>
                <Col span={8}>
                  <MetricHeatmap
                    metric={this.state.deviceLinkMetrics.getRxPacketsPerDr()!}
                    aggregation={this.state.metricsAggregation}
                    fromColor="rgb(227, 242, 253)"
                    toColor="rgb(33, 150, 243, 1)"
                  />
                </Col>
                <Col span={8}>
                  <MetricBar
                    metric={this.state.deviceLinkMetrics.getErrors()!}
                    aggregation={this.state.metricsAggregation}
                  />
                </Col>
              </Row>
            </Space>
          </Tabs.TabPane>
          <Tabs.TabPane tab="Device metrics" key="2">
            <Space direction="vertical" style={{ width: "100%" }} size="large">
              {deviceMetrics}
            </Space>
          </Tabs.TabPane>
        </Tabs>
      </Space>
    );
  }
}

export default DeviceDashboard;
