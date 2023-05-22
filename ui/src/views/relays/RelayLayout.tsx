import React, { Component } from "react";
import { Route, Switch, RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader, Menu } from "antd";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Device,
  GetDeviceRequest,
  GetDeviceResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../../stores/DeviceStore";
import ListRelayDevices from "./ListRelayDevices";

interface MatchParams {
  relayDevEui: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
  application: Application;
}

interface IState {
  relayDevice?: Device;
}

class RelayLayout extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetDeviceRequest();
    req.setDevEui(this.props.match.params.relayDevEui);

    DeviceStore.get(req, (resp: GetDeviceResponse) => {
      this.setState({
        relayDevice: resp.getDevice(),
      });
    });
  }

  render() {
    const tenant = this.props.tenant;
    const app = this.props.application;
    const rd = this.state.relayDevice;

    if (!rd) {
      return null;
    }

    let tab = "devices";

    return(
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${tenant.getId()}`}>{tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${tenant.getId()}/applications`}>Applications</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}`}>{app.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/relays`}>
                    Relays 
                  </Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>{rd.getName()}</Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={rd.getName()}
          subTitle={`relay DevEUI: ${rd.getDevEui()}`}
        />
        <Card>
          <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
            <Menu.Item key="devices">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/relays/${rd.getDevEui()}`}>
                Devices
              </Link>
            </Menu.Item>
          </Menu>
          <Switch>
            <Route
              exact
              path={this.props.match.path}
              render={props => <ListRelayDevices relayDevice={rd} {...props} />}
            />
          </Switch>
        </Card>
      </Space>
    );
  }
}

export default RelayLayout;
