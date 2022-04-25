import React, { Component } from "react";
import { Route, Switch, Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Menu, Card, Button, PageHeader } from "antd";

import { Tenant, DeleteTenantRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";
import TenantDashboard from "./TenantDashboard";
import EditTenant from "./EditTenant";

interface IState {}

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

class TenantLayout extends Component<IProps, IState> {
  deleteTenant = () => {
    let req = new DeleteTenantRequest();
    req.setId(this.props.tenant.getId());

    TenantStore.delete(req, () => {
      this.props.history.push("/tenants");
    });
  };

  render() {
    const tenant = this.props.tenant;
    const path = this.props.history.location.pathname;

    let tab = "dashboard";

    if (path.endsWith("/edit")) {
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
                <span>{tenant.getName()}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={tenant.getName()}
          subTitle={`tenant id: ${tenant.getId()}`}
          extra={[
            <Admin>
              <DeleteConfirm confirm={tenant.getName()} typ="tenant" onConfirm={this.deleteTenant}>
                <Button danger type="primary">
                  Delete tenant
                </Button>
              </DeleteConfirm>
            </Admin>,
          ]}
        />

        <Card>
          <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
            <Menu.Item key="dashboard">
              <Link to={`/tenants/${tenant.getId()}`}>Dashboard</Link>
            </Menu.Item>
            <Menu.Item key="edit">
              <Link to={`/tenants/${tenant.getId()}/edit`}>Configuration</Link>
            </Menu.Item>
          </Menu>
          <Switch>
            <Route
              exact
              path={`${this.props.match.path}`}
              render={props => <TenantDashboard tenant={tenant} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/edit`}
              render={props => <EditTenant tenant={tenant} {...props} />}
            />
          </Switch>
        </Card>
      </Space>
    );
  }
}

export default TenantLayout;
