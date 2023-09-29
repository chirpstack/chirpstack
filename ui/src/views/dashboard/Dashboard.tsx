import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import { presetPalettes } from "@ant-design/colors";
import { Space, Breadcrumb, Card, Row, Col, Empty } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import moment from "moment";
import { LatLngTuple, PointTuple } from "leaflet";
import { Popup } from "react-leaflet";
import { Doughnut } from "react-chartjs-2";

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

function GatewaysMap() {
  const [items, setItems] = useState<GatewayListItem[]>([]);

  useEffect(() => {
    let req = new ListGatewaysRequest();
    req.setLimit(9999);
    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      setItems(resp.getResultList());
    });
  }, []);

  if (items.length === 0) {
    return <Empty />;
  }

  const boundsOptions: {
    padding: PointTuple;
  } = {
    padding: [50, 50],
  };

  let bounds: LatLngTuple[] = [];
  let markers: any[] = [];

  for (const item of items) {
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

function DevicesActiveInactive({ summary }: { summary?: GetDevicesSummaryResponse }) {
  if (
    summary === undefined ||
    (summary.getNeverSeenCount() === 0 && summary.getInactiveCount() === 0 && summary.getActiveCount() === 0)
  ) {
    return <Empty />;
  }

  const data = {
    labels: ["Never seen", "Inactive", "Active"],
    datasets: [
      {
        data: [summary.getNeverSeenCount(), summary.getInactiveCount(), summary.getActiveCount()],
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

function GatewaysActiveInactive({ summary }: { summary?: GetGatewaysSummaryResponse }) {
  if (
    summary === undefined ||
    (summary.getNeverSeenCount() === 0 && summary.getOfflineCount() === 0 && summary.getOnlineCount() === 0)
  ) {
    return <Empty />;
  }

  const data = {
    labels: ["Never seen", "Offline", "Online"],
    datasets: [
      {
        data: [summary.getNeverSeenCount(), summary.getOfflineCount(), summary.getOnlineCount()],
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

function DevicesDataRates({ summary }: { summary?: GetDevicesSummaryResponse }) {
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

  if (summary === undefined || summary.getDrCountMap().toArray().length === 0) {
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

  for (const elm of summary.getDrCountMap().toArray()) {
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

function Dashboard() {
  const [gatewaysSummary, setGatewaysSummary] = useState<GetGatewaysSummaryResponse | undefined>(undefined);
  const [devicesSummary, setDevicesSummary] = useState<GetDevicesSummaryResponse | undefined>(undefined);

  useEffect(() => {
    InternalStore.getGatewaysSummary(new GetGatewaysSummaryRequest(), (resp: GetGatewaysSummaryResponse) => {
      setGatewaysSummary(resp);
    });

    InternalStore.getDevicesSummary(new GetDevicesSummaryRequest(), (resp: GetDevicesSummaryResponse) => {
      setDevicesSummary(resp);
    });
  }, []);

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Network Server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Dashboard</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Dashboard"
      />
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
        <GatewaysMap />
      </Card>
    </Space>
  );
}

export default Dashboard;
