import React, { Component } from "react";
import { Link } from "react-router-dom";

import { presetPalettes } from "@ant-design/colors";
import { Space, Breadcrumb, Card, Row, Col, PageHeader, Empty } from "antd";

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

interface GatewaysMapState {
  items: GatewayListItem[];
}

class GatewaysMap extends Component<{}, GatewaysMapState> {
  constructor(props: {}) {
    super(props);

    this.state = {
      items: [],
    };
  }

  componentDidMount() {
    this.loadData();
  }

  loadData = () => {
    let req = new ListGatewaysRequest();
    req.setLimit(9999);
    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      this.setState({
        items: resp.getResultList(),
      });
    });
  };

  render() {
    if (this.state.items.length === 0) {
      return <Empty />;
    }

    const boundsOptions: {
      padding: PointTuple;
    } = {
      padding: [50, 50],
    };

    let bounds: LatLngTuple[] = [];
    let markers: any[] = [];

    for (const item of this.state.items) {
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
}

interface GatewayProps {
  summary?: GetGatewaysSummaryResponse;
}

class GatewaysActiveInactive extends Component<GatewayProps> {
  render() {
    if (
      this.props.summary === undefined ||
      (this.props.summary.getNeverSeenCount() === 0 &&
        this.props.summary.getOfflineCount() === 0 &&
        this.props.summary.getOnlineCount() === 0)
    ) {
      return <Empty />;
    }

    const data = {
      labels: ["Never seen", "Offline", "Online"],
      datasets: [
        {
          data: [
            this.props.summary.getNeverSeenCount(),
            this.props.summary.getOfflineCount(),
            this.props.summary.getOnlineCount(),
          ],
          backgroundColor: [presetPalettes.orange.primary, presetPalettes.red.primary, presetPalettes.green.primary],
        },
      ],
    };

    const options: {
      animation: false;
    } = {
      animation: false,
    };

    return <Doughnut data={data} options={options} className="chart-doughtnut" />;
  }
}

interface DeviceProps {
  summary?: GetDevicesSummaryResponse;
}

class DevicesActiveInactive extends Component<DeviceProps> {
  render() {
    if (
      this.props.summary === undefined ||
      (this.props.summary.getNeverSeenCount() === 0 &&
        this.props.summary.getInactiveCount() === 0 &&
        this.props.summary.getActiveCount() === 0)
    ) {
      return <Empty />;
    }

    const data = {
      labels: ["Never seen", "Inactive", "Active"],
      datasets: [
        {
          data: [
            this.props.summary.getNeverSeenCount(),
            this.props.summary.getInactiveCount(),
            this.props.summary.getActiveCount(),
          ],
          backgroundColor: [presetPalettes.orange.primary, presetPalettes.red.primary, presetPalettes.green.primary],
        },
      ],
    };

    const options: {
      animation: false;
    } = {
      animation: false,
    };

    return <Doughnut data={data} options={options} className="chart-doughtnut" />;
  }
}

class DevicesDataRates extends Component<DeviceProps> {
  getColor = (dr: number) => {
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

  render() {
    if (this.props.summary === undefined || this.props.summary.getDrCountMap().toArray().length === 0) {
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

    for (const elm of this.props.summary.getDrCountMap().toArray()) {
      data.labels.push(`DR${elm[0]}`);
      data.datasets[0].data.push(elm[1]);
      data.datasets[0].backgroundColor.push(this.getColor(elm[0]));
    }

    const options: {
      animation: false;
    } = {
      animation: false,
    };

    return <Doughnut data={data} options={options} className="chart-doughtnut" />;
  }
}

interface IProps {}

interface IState {
  gatewaysSummary?: GetGatewaysSummaryResponse;
  devicesSummary?: GetDevicesSummaryResponse;
}

class Dashboard extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    InternalStore.getGatewaysSummary(new GetGatewaysSummaryRequest(), (resp: GetGatewaysSummaryResponse) => {
      this.setState({
        gatewaysSummary: resp,
      });
    });

    InternalStore.getDevicesSummary(new GetDevicesSummaryRequest(), (resp: GetDevicesSummaryResponse) => {
      this.setState({
        devicesSummary: resp,
      });
    });
  }

  render() {
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
              <DevicesActiveInactive summary={this.state.devicesSummary} />
            </Card>
          </Col>
          <Col span={8}>
            <Card title="Active gateways">
              <GatewaysActiveInactive summary={this.state.gatewaysSummary} />
            </Card>
          </Col>
          <Col span={8}>
            <Card title="Device data-rate usage">
              <DevicesDataRates summary={this.state.devicesSummary} />
            </Card>
          </Col>
        </Row>
        <Card title="Gateway map">
          <GatewaysMap />
        </Card>
      </Space>
    );
  }
}

export default Dashboard;
