import React, { Component } from "react";
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

class GatewaysMap extends Component<GatewaysMapProps> {
  render() {
    if (this.props.items.length === 0) {
      return <Empty />;
    }

    const boundsOptions: {
      padding: PointTuple;
    } = {
      padding: [50, 50],
    };

    let bounds: LatLngTuple[] = [];
    let markers: any[] = [];

    for (const item of this.props.items) {
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

interface IProps {
  tenant: Tenant;
}

interface IState {
  gatewayItems: GatewayListItem[];
  gatewaysSummary?: GetGatewaysSummaryResponse;
  devicesSummary?: GetDevicesSummaryResponse;
}

class TenantDashboard extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      gatewayItems: [],
    };
  }

  componentDidMount() {
    this.loadData();
  }

  componentDidUpdate(oldProps: IProps) {
    if (oldProps === this.props) {
      return;
    }

    this.loadData();
  }

  loadData = () => {
    {
      let req = new GetGatewaysSummaryRequest();
      req.setTenantId(this.props.tenant.getId());

      InternalStore.getGatewaysSummary(req, (resp: GetGatewaysSummaryResponse) => {
        this.setState({
          gatewaysSummary: resp,
        });
      });
    }

    {
      let req = new GetDevicesSummaryRequest();
      req.setTenantId(this.props.tenant.getId());

      InternalStore.getDevicesSummary(req, (resp: GetDevicesSummaryResponse) => {
        this.setState({
          devicesSummary: resp,
        });
      });
    }

    {
      let req = new ListGatewaysRequest();
      req.setTenantId(this.props.tenant.getId());
      req.setLimit(9999);

      GatewayStore.list(req, (resp: ListGatewaysResponse) => {
        this.setState({
          gatewayItems: resp.getResultList(),
        });
      });
    }
  };

  render() {
    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
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
          <GatewaysMap items={this.state.gatewayItems} />
        </Card>
      </Space>
    );
  }
}

export default TenantDashboard;
