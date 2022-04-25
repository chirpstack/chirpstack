import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Tenant, TenantUser, AddTenantUserRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantUserForm from "./TenantUserForm";
import TenantStore from "../../stores/TenantStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

class CreateTenantUser extends Component<IProps> {
  onFinish = (obj: TenantUser) => {
    obj.setTenantId(this.props.tenant.getId());

    let req = new AddTenantUserRequest();
    req.setTenantUser(obj);

    TenantStore.addUser(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/users`);
    });
  };

  render() {
    const tu = new TenantUser();

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
                  <Link to={`/tenants/${this.props.tenant.getId()}/users`}>Tenant users</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add tenant user"
        />
        <Card>
          <TenantUserForm initialValues={tu} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateTenantUser;
