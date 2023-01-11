import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Gateway, CreateGatewayRequest } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import GatewayForm from "./GatewayForm";
import GatewayStore from "../../stores/GatewayStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

class CreateGateway extends Component<IProps> {
  onFinish = (obj: Gateway) => {
    obj.setTenantId(this.props.tenant.getId());

    let req = new CreateGatewayRequest();
    req.setGateway(obj);

    GatewayStore.create(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/gateways/${obj.getGatewayId()}`);
    });
  };

  render() {
    let gateway = new Gateway();
    gateway.setStatsInterval(30);

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          title="Add gateway"
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
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
        />
        <Card>
          <GatewayForm initialValues={gateway} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateGateway;
