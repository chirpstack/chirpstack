import React, { Component } from "react";
import { Route, Switch, RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader, Menu } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Gateway,
  GetGatewayRequest,
  GetGatewayResponse,
  DeleteGatewayRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import GatewayStore from "../../stores/GatewayStore";
import DeleteConfirm from "../../components/DeleteConfirm";

import GatewayDashboard from "./GatewayDashboard";
import EditGateway from "./EditGateway";
import GatewayFrames from "./GatewayFrames";
import GatewayCertificate from "./GatewayCertificate";
import Admin from "../../components/Admin";
import SessionStore from "../../stores/SessionStore";

interface MatchParams {
  gatewayId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
}

interface IState {
  gateway?: Gateway;
  lastSeenAt?: Date;
}

class GatewayLayout extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.getGateway();
  }

  getGateway = () => {
    let req = new GetGatewayRequest();
    req.setGatewayId(this.props.match.params.gatewayId);

    GatewayStore.get(req, (resp: GetGatewayResponse) => {
      this.setState({
        gateway: resp.getGateway(),
      });

      if (resp.getLastSeenAt() !== undefined) {
        this.setState({
          lastSeenAt: resp.getLastSeenAt()!.toDate(),
        });
      }
    });
  };

  deleteGateway = () => {
    let req = new DeleteGatewayRequest();
    req.setGatewayId(this.props.match.params.gatewayId);

    GatewayStore.delete(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/gateways`);
    });
  };

  render() {
    const tenant = this.props.tenant;
    const gw = this.state.gateway;
    if (!gw) {
      return null;
    }

    const path = this.props.history.location.pathname;
    let tab = "dashboard";

    if (path.endsWith("/edit")) {
      tab = "edit";
    }
    if (path.endsWith("/certificate")) {
      tab = "cert";
    }
    if (path.endsWith("/frames")) {
      tab = "frames";
    }

    let isGatewayAdmin =
      SessionStore.isAdmin() ||
      SessionStore.isTenantAdmin(this.props.tenant.getId()) ||
      SessionStore.isTenantGatewayAdmin(this.props.tenant.getId());

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
                  <Link to={`/tenants/${this.props.tenant.getId()}/gateways`}>Gateways</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>{gw.getName()}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={gw.getName()}
          subTitle={`gateway id: ${gw.getGatewayId()}`}
          extra={[
            <Admin tenantId={this.props.tenant.getId()} isGatewayAdmin>
              <DeleteConfirm confirm={gw.getName()} typ="gateway" onConfirm={this.deleteGateway}>
                <Button danger type="primary">
                  Delete gateway
                </Button>
              </DeleteConfirm>
            </Admin>,
          ]}
        />
        <Card>
          <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
            <Menu.Item key="dashboard">
              <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}`}>Dashboard</Link>
            </Menu.Item>
            <Menu.Item key="edit">
              <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}/edit`}>Configuration</Link>
            </Menu.Item>
            {isGatewayAdmin && (
              <Menu.Item key="cert">
                <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}/certificate`}>TLS certificate</Link>
              </Menu.Item>
            )}
            <Menu.Item key="frames">
              <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}/frames`}>LoRaWAN frames</Link>
            </Menu.Item>
          </Menu>
          <Switch>
            <Route
              exact
              path={`${this.props.match.path}`}
              render={props => <GatewayDashboard gateway={gw} lastSeenAt={this.state.lastSeenAt} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/edit`}
              render={props => <EditGateway gateway={gw} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/certificate`}
              render={props => <GatewayCertificate gateway={gw} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/frames`}
              render={props => <GatewayFrames gateway={gw} {...props} />}
            />
          </Switch>
        </Card>
      </Space>
    );
  }
}

export default GatewayLayout;
