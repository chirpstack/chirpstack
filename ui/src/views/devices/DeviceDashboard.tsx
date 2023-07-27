import React, { useState, useEffect } from "react";
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

function DeviceDashboard(props: IProps) {
  const [metricsAggregation, setMetricsAggregation] = useState<Aggregation>(Aggregation.DAY);
  const [deviceMetrics, setDeviceMetrics] = useState<GetDeviceMetricsResponse | undefined>(undefined);
  const [deviceLinkMetrics, setDeviceLinkMetrics] = useState<GetDeviceLinkMetricsResponse | undefined>(undefined);
  const [deviceLinkMetricsLoaded, setDeviceLinkMetricsLoaded] = useState<boolean>(false);

  useEffect(() => {
    loadMetrics();
  }, [props, metricsAggregation]);

  const loadMetrics = () => {
    const agg = metricsAggregation;
    const end = moment();
    let start = moment();

    if (agg === Aggregation.DAY) {
      start = start.subtract(30, "days");
    } else if (agg === Aggregation.HOUR) {
      start = start.subtract(24, "hours");
    } else if (agg === Aggregation.MONTH) {
      start = start.subtract(12, "months");
    }

    setDeviceLinkMetricsLoaded(false);
    loadLinkMetrics(start.toDate(), end.toDate(), agg);
    loadDeviceMetrics(start.toDate(), end.toDate(), agg);
  };

  const loadDeviceMetrics = (start: Date, end: Date, agg: Aggregation) => {
    let startPb = new Timestamp();
    let endPb = new Timestamp();

    startPb.fromDate(start);
    endPb.fromDate(end);

    let req = new GetDeviceMetricsRequest();
    req.setDevEui(props.device.getDevEui());
    req.setStart(startPb);
    req.setEnd(endPb);
    req.setAggregation(agg);

    DeviceStore.getMetrics(req, (resp: GetDeviceMetricsResponse) => {
      setDeviceMetrics(resp);
    });
  };

  const loadLinkMetrics = (start: Date, end: Date, agg: Aggregation) => {
    let startPb = new Timestamp();
    let endPb = new Timestamp();

    startPb.fromDate(start);
    endPb.fromDate(end);

    let req = new GetDeviceLinkMetricsRequest();
    req.setDevEui(props.device.getDevEui());
    req.setStart(startPb);
    req.setEnd(endPb);
    req.setAggregation(agg);

    DeviceStore.getLinkMetrics(req, (resp: GetDeviceLinkMetricsResponse) => {
      setDeviceLinkMetrics(resp);
      setDeviceLinkMetricsLoaded(true);
    });
  };

  const onMetricsAggregationChange = (e: RadioChangeEvent) => {
    setMetricsAggregation(e.target.value);
  };

  if (deviceLinkMetrics === undefined || deviceMetrics === undefined) {
    return null;
  }

  let dm = [];

  {
    let states = deviceMetrics.getStatesMap();
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

      dm.push(<Row gutter={24}>{items}</Row>);
    }
  }

  {
    let metrics = deviceMetrics.getMetricsMap();
    let keys = metrics.toArray().map(v => v[0]);
    keys.sort();

    for (let i = 0; i < keys.length; i += 3) {
      let items = keys.slice(i, i + 3).map(k => {
        let m = metrics.get(k)!;
        return (
          <Col span={8}>
            <MetricChart metric={m} aggregation={metricsAggregation} zeroToNull />
          </Col>
        );
      });

      dm.push(<Row gutter={24}>{items}</Row>);
    }
  }

  let lastSeenAt = "Never";
  if (props.lastSeenAt !== undefined) {
    lastSeenAt = moment(props.lastSeenAt).format("YYYY-MM-DD HH:mm:ss");
  }

  const loading = !deviceLinkMetricsLoaded || !deviceMetrics;

  const aggregations = (
    <Space direction="horizontal">
      {loading && <Spin size="small" />}
      <Radio.Group value={metricsAggregation} onChange={onMetricsAggregationChange} size="small">
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
      <Button type="primary" size="small" icon={<ReloadOutlined />} onClick={loadMetrics} disabled={loading} />
    </Space>
  );

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Card>
        <Descriptions>
          <Descriptions.Item label="Last seen">{lastSeenAt}</Descriptions.Item>
          <Descriptions.Item label="Device profile">
            <Link
              to={`/tenants/${props.deviceProfile.getTenantId()}/device-profiles/${props.deviceProfile.getId()}/edit`}
            >
              {props.deviceProfile.getName()}
            </Link>
          </Descriptions.Item>
          <Descriptions.Item label="Enabled">{props.device.getIsDisabled() ? "no" : "yes"}</Descriptions.Item>
          <Descriptions.Item label="Description">{props.device.getDescription()}</Descriptions.Item>
        </Descriptions>
      </Card>
      <Tabs tabBarExtraContent={aggregations}>
        <Tabs.TabPane tab="Link metrics" key="1">
          <Space direction="vertical" style={{ width: "100%" }} size="large">
            <Row gutter={24}>
              <Col span={8}>
                <MetricChart metric={deviceLinkMetrics.getRxPackets()!} aggregation={metricsAggregation} />
              </Col>
              <Col span={8}>
                <MetricChart metric={deviceLinkMetrics.getGwRssi()!} aggregation={metricsAggregation} zeroToNull />
              </Col>
              <Col span={8}>
                <MetricChart metric={deviceLinkMetrics.getGwSnr()!} aggregation={metricsAggregation} zeroToNull />
              </Col>
            </Row>
            <Row gutter={24}>
              <Col span={8}>
                <MetricHeatmap
                  metric={deviceLinkMetrics.getRxPacketsPerFreq()!}
                  aggregation={metricsAggregation}
                  fromColor="rgb(227, 242, 253)"
                  toColor="rgb(33, 150, 243, 1)"
                />
              </Col>
              <Col span={8}>
                <MetricHeatmap
                  metric={deviceLinkMetrics.getRxPacketsPerDr()!}
                  aggregation={metricsAggregation}
                  fromColor="rgb(227, 242, 253)"
                  toColor="rgb(33, 150, 243, 1)"
                />
              </Col>
              <Col span={8}>
                <MetricBar metric={deviceLinkMetrics.getErrors()!} aggregation={metricsAggregation} />
              </Col>
            </Row>
          </Space>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Device metrics" key="2">
          <Space direction="vertical" style={{ width: "100%" }} size="large">
            {dm}
          </Space>
        </Tabs.TabPane>
      </Tabs>
    </Space>
  );
}

export default DeviceDashboard;
