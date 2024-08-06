import { useState, useEffect } from "react";

import { format, sub } from "date-fns";
import { Descriptions, Space, Card, Row, Col } from "antd";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import type {
  Gateway,
  GetGatewayMetricsResponse,
  GetGatewayDutyCycleMetricsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import {
  GetGatewayMetricsRequest,
  GetGatewayDutyCycleMetricsRequest,
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

function GatewayDashboard(props: IProps) {
  const [metricsAggregation] = useState<Aggregation>(Aggregation.DAY);
  const [gatewayMetrics, setGatewayMetrics] = useState<GetGatewayMetricsResponse | undefined>(undefined);
  const [gatewayDutyCycleMetrics, setGatewayDutyCycleMetrics] = useState<
    GetGatewayDutyCycleMetricsResponse | undefined
  >(undefined);

  useEffect(() => {
    const agg = metricsAggregation;
    const end = new Date();
    let start = new Date();

    if (agg === Aggregation.DAY) {
      start = sub(start, { days: 30 });
    } else if (agg === Aggregation.HOUR) {
      start = sub(start, { hours: 24 });
    } else if (agg === Aggregation.MONTH) {
      start = sub(start, { months: 12 });
    }

    const startPb = new Timestamp();
    const endPb = new Timestamp();

    startPb.fromDate(start);
    endPb.fromDate(end);

    const req = new GetGatewayMetricsRequest();
    req.setGatewayId(props.gateway.getGatewayId());
    req.setStart(startPb);
    req.setEnd(endPb);
    req.setAggregation(agg);

    GatewayStore.getMetrics(req, (resp: GetGatewayMetricsResponse) => {
      setGatewayMetrics(resp);
    });

    const dcEnd = sub(new Date(), { minutes: 1 });
    const dcEndPb = new Timestamp();
    dcEndPb.fromDate(dcEnd);

    const dcStart = sub(dcEnd, { hours: 1 });
    const dcStartPb = new Timestamp();
    dcStartPb.fromDate(dcStart);

    const dcReq = new GetGatewayDutyCycleMetricsRequest();
    dcReq.setGatewayId(props.gateway.getGatewayId());
    dcReq.setStart(dcStartPb);
    dcReq.setEnd(dcEndPb);

    GatewayStore.getDutyCycleMetrics(dcReq, (resp: GetGatewayDutyCycleMetricsResponse) => {
      setGatewayDutyCycleMetrics(resp);
    });
  }, [props, metricsAggregation]);

  const loc = props.gateway.getLocation()!;
  const location: [number, number] = [loc.getLatitude(), loc.getLongitude()];

  if (gatewayMetrics === undefined || gatewayDutyCycleMetrics === undefined) {
    return null;
  }

  let lastSeenAt: string = "Never";
  if (props.lastSeenAt !== undefined) {
    lastSeenAt = format(props.lastSeenAt, "yyyy-MM-dd HH:mm:ss");
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Card>
        <Descriptions>
          <Descriptions.Item label="Last seen">{lastSeenAt}</Descriptions.Item>
          <Descriptions.Item label="Region ID">
            {props.gateway.getMetadataMap().get("region_config_id")}
          </Descriptions.Item>
          <Descriptions.Item label="Region common-name">
            {props.gateway.getMetadataMap().get("region_common_name")}
          </Descriptions.Item>
          <Descriptions.Item label="Description">{props.gateway.getDescription()}</Descriptions.Item>
        </Descriptions>
      </Card>
      <Row gutter={24}>
        <Col span={24}>
          <Map height={500} center={location}>
            <Marker position={location} faIcon="wifi" color="blue" />
          </Map>
        </Col>
      </Row>
      {gatewayDutyCycleMetrics.getMaxLoadPercentage()!.getDatasetsList().length !== 0 &&
        gatewayDutyCycleMetrics.getWindowPercentage()!.getDatasetsList().length !== 0 && (
          <Row gutter={24}>
            <Col span={12}>
              <MetricChart metric={gatewayDutyCycleMetrics.getWindowPercentage()!} aggregation={Aggregation.MINUTE} />
            </Col>
            <Col span={12}>
              <MetricChart metric={gatewayDutyCycleMetrics.getMaxLoadPercentage()!} aggregation={Aggregation.MINUTE} />
            </Col>
          </Row>
        )}
      <Row gutter={24}>
        <Col span={8}>
          <MetricChart metric={gatewayMetrics.getRxPackets()!} aggregation={metricsAggregation} />
        </Col>
        <Col span={8}>
          <MetricChart metric={gatewayMetrics.getTxPackets()!} aggregation={metricsAggregation} />
        </Col>
        <Col span={8}>
          <MetricHeatmap
            metric={gatewayMetrics.getRxPacketsPerFreq()!}
            aggregation={metricsAggregation}
            fromColor="rgb(227, 242, 253)"
            toColor="rgb(33, 150, 243, 1)"
          />
        </Col>
      </Row>
      <Row gutter={24}>
        <Col span={8}>
          <MetricHeatmap
            metric={gatewayMetrics.getTxPacketsPerFreq()!}
            aggregation={metricsAggregation}
            fromColor="rgb(227, 242, 253)"
            toColor="rgb(33, 150, 243, 1)"
          />
        </Col>
        <Col span={8}>
          <MetricHeatmap
            metric={gatewayMetrics.getRxPacketsPerDr()!}
            aggregation={metricsAggregation}
            fromColor="rgb(227, 242, 253)"
            toColor="rgb(33, 150, 243, 1)"
          />
        </Col>
        <Col span={8}>
          <MetricHeatmap
            metric={gatewayMetrics.getTxPacketsPerDr()!}
            aggregation={metricsAggregation}
            fromColor="rgb(227, 242, 253)"
            toColor="rgb(33, 150, 243, 1)"
          />
        </Col>
      </Row>
      <Row gutter={24}>
        <Col span={8}>
          <MetricBar metric={gatewayMetrics.getTxPacketsPerStatus()!} aggregation={metricsAggregation} />
        </Col>
      </Row>
    </Space>
  );
}

export default GatewayDashboard;
