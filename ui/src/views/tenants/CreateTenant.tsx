import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Tenant, CreateTenantRequest, CreateTenantResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantForm from "./TenantForm";
import TenantStore from "../../stores/TenantStore";

class CreateTenant extends Component<RouteComponentProps> {
  onFinish = (obj: Tenant) => {
    let req = new CreateTenantRequest();
    req.setTenant(obj);

    TenantStore.create(req, (resp: CreateTenantResponse) => {
      this.props.history.push("/tenants/" + resp.getId());
    });
  };

  render() {
    const tenant = new Tenant();

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Network-server</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to="/tenants">Tenants</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add tenant"
        />
        <Card>
          <TenantForm initialValues={tenant} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateTenant;
