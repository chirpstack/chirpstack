import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { presetPalettes } from "@ant-design/colors";
import { Card, Col, Row, Space, Empty } from "antd";

import moment from "moment";
import { LatLngTuple, PointTuple } from "leaflet";
import { Popup } from "react-leaflet";
import { Doughnut } from "react-chartjs-2";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import {
  GetGatewaysSummaryRequest,
  GetGatewaysSummaryResponse,
  GetDevicesSummaryRequest,
  GetDevicesSummaryResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import {
  ListGatewaysRequest,
  ListGatewaysResponse,
  GatewayListItem,
  GatewayState,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import InternalStore from "../../stores/InternalStore";
import GatewayStore from "../../stores/GatewayStore";
import Map, { Marker, MarkerColor } from "../../components/Map";

interface GatewaysMapProps {
  items: GatewayListItem[];
}

function GatewaysMap(props: GatewaysMapProps) {
  if (props.items.length === 0) {
    return <Empty />;
  }

  const boundsOptions: {
    padding: PointTuple;
  } = {
    padding: [50, 50],
  };

  let bounds: LatLngTuple[] = [];
  let markers: any[] = [];

  for (const item of props.items) {
    if (item.getLocation() === undefined) {
      continue;
    }

    const pos: LatLngTuple = [item.getLocation()!.getLatitude(), item.getLocation()!.getLongitude()];
    bounds.push(pos);

    let color: MarkerColor = "orange";
    let lastSeen: string = "Never seen online";

    if (item.getState() === GatewayState.OFFLINE) {
      color = "red";
    } else if (item.getState() === GatewayState.ONLINE) {
      color = "green";
    }

    if (item.getLastSeenAt() !== undefined) {
      let ts = moment(item.getLastSeenAt()!.toDate());
      lastSeen = ts.fromNow();
    }

    markers.push(
      <Marker position={pos} faIcon="wifi" color={color}>
        <Popup>
          <Link to={`/tenants/${item.getTenantId()}/gateways/${item.getGatewayId()}`}>{item.getName()}</Link>
          <br />
          {item.getGatewayId()}
          <br />
          <br />
          {lastSeen}
        </Popup>
      </Marker>,
    );
  }

  return (
    <Map height={500} bounds={bounds} boundsOptions={boundsOptions}>
      {markers}
    </Map>
  );
}

interface GatewayProps {
  summary?: GetGatewaysSummaryResponse;
}

function GatewaysActiveInactive(props: GatewayProps) {
  if (
    props.summary === undefined ||
    (props.summary.getNeverSeenCount() === 0 &&
      props.summary.getOfflineCount() === 0 &&
      props.summary.getOnlineCount() === 0)
  ) {
    return <Empty />;
  }

  const data = {
    labels: ["Never seen", "Offline", "Online"],
    datasets: [
      {
        data: [props.summary.getNeverSeenCount(), props.summary.getOfflineCount(), props.summary.getOnlineCount()],
        backgroundColor: [presetPalettes.orange.primary, presetPalettes.red.primary, presetPalettes.green.primary],
      },
    ],
  };

  const options: {
    animation: boolean;
    responsive: boolean;
    maintainAspectRatio: boolean;
  } = {
    animation: false,
    responsive: true,
    maintainAspectRatio: false,
  };

  return <div className="chart-doughnut"><Doughnut data={data} options={options} /></div>;
}

interface DeviceProps {
  summary?: GetDevicesSummaryResponse;
}

function DevicesActiveInactive(props: DeviceProps) {
  if (
    props.summary === undefined ||
    (props.summary.getNeverSeenCount() === 0 &&
      props.summary.getInactiveCount() === 0 &&
      props.summary.getActiveCount() === 0)
  ) {
    return <Empty />;
  }

  const data = {
    labels: ["Never seen", "Inactive", "Active"],
    datasets: [
      {
        data: [props.summary.getNeverSeenCount(), props.summary.getInactiveCount(), props.summary.getActiveCount()],
        backgroundColor: [presetPalettes.orange.primary, presetPalettes.red.primary, presetPalettes.green.primary],
      },
    ],
  };

  const options: {
    animation: boolean;
    responsive: boolean;
    maintainAspectRatio: boolean;
  } = {
    animation: false,
    responsive: true,
    maintainAspectRatio: false,
  };

  return <div className="chart-doughnut"><Doughnut data={data} options={options} /></div>;
}

function DevicesDataRates(props: DeviceProps) {
  const getColor = (dr: number) => {
    return [
      "#ff5722",
      "#ff9800",
      "#ffc107",
      "#ffeb3b",
      "#cddc39",
      "#8bc34a",
      "#4caf50",
      "#009688",
      "#00bcd4",
      "#03a9f4",
      "#2196f3",
      "#3f51b5",
      "#673ab7",
      "#9c27b0",
      "#e91e63",
    ][dr];
  };

  if (props.summary === undefined || props.summary.getDrCountMap().toArray().length === 0) {
    return <Empty />;
  }

  let data: {
    labels: string[];
    datasets: {
      data: number[];
      backgroundColor: string[];
    }[];
  } = {
    labels: [],
    datasets: [
      {
        data: [],
        backgroundColor: [],
      },
    ],
  };

  for (const elm of props.summary.getDrCountMap().toArray()) {
    data.labels.push(`DR${elm[0]}`);
    data.datasets[0].data.push(elm[1]);
    data.datasets[0].backgroundColor.push(getColor(elm[0]));
  }

  const options: {
    animation: boolean;
    responsive: boolean;
    maintainAspectRatio: boolean;
  } = {
    animation: false,
    responsive: true,
    maintainAspectRatio: false,
  };

  return <div className="chart-doughnut"><Doughnut data={data} options={options} /></div>;
}

function TenantDashboard({ tenant }: { tenant: Tenant }) {
  const [gatewayItems, setGatewayItems] = useState<GatewayListItem[]>([]);
  const [gatewaysSummary, setGatewaysSummary] = useState<GetGatewaysSummaryResponse | undefined>(undefined);
  const [devicesSummary, setDevicesSummary] = useState<GetDevicesSummaryResponse | undefined>(undefined);

  useEffect(() => {
    {
      let req = new GetGatewaysSummaryRequest();
      req.setTenantId(tenant.getId());

      InternalStore.getGatewaysSummary(req, (resp: GetGatewaysSummaryResponse) => {
        setGatewaysSummary(resp);
      });
    }

    {
      let req = new GetDevicesSummaryRequest();
      req.setTenantId(tenant.getId());

      InternalStore.getDevicesSummary(req, (resp: GetDevicesSummaryResponse) => {
        setDevicesSummary(resp);
      });
    }

    {
      let req = new ListGatewaysRequest();
      req.setTenantId(tenant.getId());
      req.setLimit(9999);

      GatewayStore.list(req, (resp: ListGatewaysResponse) => {
        setGatewayItems(resp.getResultList());
      });
    }
  }, [tenant]);

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Row gutter={24}>
        <Col span={8}>
          <Card title="Active devices">
            <DevicesActiveInactive summary={devicesSummary} />
          </Card>
        </Col>
        <Col span={8}>
          <Card title="Active gateways">
            <GatewaysActiveInactive summary={gatewaysSummary} />
          </Card>
        </Col>
        <Col span={8}>
          <Card title="Device data-rate usage">
            <DevicesDataRates summary={devicesSummary} />
          </Card>
        </Col>
      </Row>
      <Card title="Gateway map">
        <GatewaysMap items={gatewayItems} />
      </Card>
    </Space>
  );
}

export default TenantDashboard;
