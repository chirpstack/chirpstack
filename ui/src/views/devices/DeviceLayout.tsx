import React, { Component } from "react";
import { Route, Switch, RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader, Menu } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  DeviceProfile,
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  Device,
  GetDeviceRequest,
  GetDeviceResponse,
  DeleteDeviceRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../../stores/DeviceStore";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import DeleteConfirm from "../../components/DeleteConfirm";

import DeviceDashboard from "./DeviceDashboard";
import EditDevice from "./EditDevice";
import SetDeviceKeys from "./SetDeviceKeys";
import DeviceFrames from "./DeviceFrames";
import DeviceEvents from "./DeviceEvents";
import DeviceQueue from "./DeviceQueue";
import DeviceActivation from "./DeviceActivation";

interface MatchParams {
  devEui: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
  application: Application;
}

interface IState {
  device?: Device;
  deviceProfile?: DeviceProfile;
  lastSeenAt?: Date;
}

class DeviceLayout extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.getDevice(this.getDeviceProfile);
  }

  getDevice = (cb: () => void) => {
    let req = new GetDeviceRequest();
    req.setDevEui(this.props.match.params.devEui);

    DeviceStore.get(req, (resp: GetDeviceResponse) => {
      this.setState(
        {
          device: resp.getDevice(),
        },
        cb,
      );

      if (resp.getLastSeenAt() !== undefined) {
        this.setState({
          lastSeenAt: resp.getLastSeenAt()!.toDate(),
        });
      }
    });
  };

  getDeviceProfile = () => {
    let req = new GetDeviceProfileRequest();
    req.setId(this.state.device!.getDeviceProfileId());

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      this.setState({
        deviceProfile: resp.getDeviceProfile(),
      });
    });
  };

  deleteDevice = () => {
    let req = new DeleteDeviceRequest();
    req.setDevEui(this.props.match.params.devEui);

    DeviceStore.delete(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}`);
    });
  };

  render() {
    const device = this.state.device;
    const dp = this.state.deviceProfile;
    if (!device || !dp) {
      return null;
    }

    const tenant = this.props.tenant;
    const app = this.props.application;

    const path = this.props.history.location.pathname;
    let tab = "dashboard";

    if (path.endsWith("edit")) {
      tab = "edit";
    }
    if (path.endsWith("queue")) {
      tab = "queue";
    }
    if (path.endsWith("keys")) {
      tab = "keys";
    }
    if (path.endsWith("activation")) {
      tab = "activation";
    }
    if (path.endsWith("events")) {
      tab = "events";
    }
    if (path.endsWith("frames")) {
      tab = "frames";
    }

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications`}>Applications</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}`}>
                    {this.props.application.getName()}
                  </Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}`}>
                    Devices
                  </Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>{device.getName()}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={device.getName()}
          subTitle={`device eui: ${device.getDevEui()}`}
          extra={[
            <DeleteConfirm typ="device" confirm={device.getName()} onConfirm={this.deleteDevice}>
              <Button danger type="primary">
                Delete device
              </Button>
            </DeleteConfirm>,
          ]}
        />
        <Card>
          <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
            <Menu.Item key="dashboard">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}`}>
                Dashboard
              </Link>
            </Menu.Item>
            <Menu.Item key="edit">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/edit`}>
                Configuration
              </Link>
            </Menu.Item>
            <Menu.Item key="keys" disabled={!dp.getSupportsOtaa()}>
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/keys`}>
                OTAA keys
              </Link>
            </Menu.Item>
            <Menu.Item key="activation">
              <Link
                to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/activation`}
              >
                Activation
              </Link>
            </Menu.Item>
            <Menu.Item key="queue">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/queue`}>
                Queue
              </Link>
            </Menu.Item>
            <Menu.Item key="events">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/events`}>
                Events
              </Link>
            </Menu.Item>
            <Menu.Item key="frames">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/frames`}>
                LoRaWAN frames
              </Link>
            </Menu.Item>
          </Menu>
          <Switch>
            <Route
              exact
              path={this.props.match.path}
              render={props => (
                <DeviceDashboard device={device} lastSeenAt={this.state.lastSeenAt} deviceProfile={dp} {...props} />
              )}
            />
            <Route
              exact
              path={`${this.props.match.path}/edit`}
              render={props => <EditDevice device={device} application={app} tenant={tenant} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/keys`}
              render={props => (
                <SetDeviceKeys device={device} application={app} tenant={tenant} deviceProfile={dp} {...props} />
              )}
            />
            <Route
              exact
              path={`${this.props.match.path}/frames`}
              render={props => <DeviceFrames device={device} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/events`}
              render={props => <DeviceEvents device={device} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/queue`}
              render={props => <DeviceQueue device={device} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/activation`}
              render={props => (
                <DeviceActivation device={device} deviceProfile={dp} tenant={tenant} application={app} {...props} />
              )}
            />
          </Switch>
        </Card>
      </Space>
    );
  }
}

export default DeviceLayout;
