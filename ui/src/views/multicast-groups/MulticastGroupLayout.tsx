import React, { Component } from "react";
import { Route, Switch, RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader, Menu } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetMulticastGroupRequest,
  GetMulticastGroupResponse,
  MulticastGroup,
  DeleteMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupStore from "../../stores/MulticastGroupStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import ListMulticastGroupDevices from "./ListMulticastGroupDevices";
import ListMulticastGroupGateways from "./ListMulticastGroupGateways";
import EditMulticastGroup from "./EditMulticastGroup";
import Admin from "../../components/Admin";

interface MatchParams {
  multicastGroupId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
  application: Application;
}

interface IState {
  multicastGroup?: MulticastGroup;
}

class MulticastGroupLayout extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetMulticastGroupRequest();
    req.setId(this.props.match.params.multicastGroupId);

    MulticastGroupStore.get(req, (resp: GetMulticastGroupResponse) => {
      this.setState({
        multicastGroup: resp.getMulticastGroup(),
      });
    });
  }

  deleteMulticastGroup = () => {
    let req = new DeleteMulticastGroupRequest();
    req.setId(this.props.match.params.multicastGroupId);

    MulticastGroupStore.delete(req, () => {
      this.props.history.push(
        `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/multicast-groups`,
      );
    });
  };

  render() {
    const tenant = this.props.tenant;
    const app = this.props.application;
    const mg = this.state.multicastGroup;

    if (!mg) {
      return null;
    }

    let tab = "devices";

    const path = this.props.history.location.pathname;
    if (path.endsWith("gateways")) {
      tab = "gateways";
    }
    if (path.endsWith("edit")) {
      tab = "edit";
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
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications/${app.getId()}`}>{app.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications/${app.getId()}/multicast-groups`}>
                    Multicast-groups
                  </Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>{mg.getName()}</Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={mg.getName()}
          subTitle={`multicast-group id: ${mg.getId()}`}
          extra={[
            <Admin tenantId={tenant.getId()} isDeviceAdmin>
              <DeleteConfirm typ="multicast-group" confirm={mg.getName()} onConfirm={this.deleteMulticastGroup}>
                <Button danger type="primary">
                  Delete multicast-group
                </Button>
              </DeleteConfirm>
            </Admin>,
          ]}
        />
        <Card>
          <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
            <Menu.Item key="devices">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}`}>
                Devices
              </Link>
            </Menu.Item>
            <Menu.Item key="gateways">
              <Link
                to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/gateways`}
              >
                Gateways
              </Link>
            </Menu.Item>
            <Menu.Item key="edit">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/edit`}>
                Configuration
              </Link>
            </Menu.Item>
          </Menu>
          <Switch>
            <Route
              exact
              path={this.props.match.path}
              render={props => <ListMulticastGroupDevices multicastGroup={mg} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/gateways`}
              render={props => <ListMulticastGroupGateways application={app} multicastGroup={mg} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/edit`}
              render={props => <EditMulticastGroup application={app} multicastGroup={mg} {...props} />}
            />
          </Switch>
        </Card>
      </Space>
    );
  }
}

export default MulticastGroupLayout;
