import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { ApiKey, CreateApiKeyRequest, CreateApiKeyResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import ApiKeyForm from "./ApiKeyForm";
import ApiKeyToken from "./ApiKeyToken";
import InternalStore from "../../stores/InternalStore";

interface IState {
  createApiKeyResponse?: CreateApiKeyResponse;
}

interface IProps {
  tenant: Tenant;
}

class CreateTenantApiKey extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  onFinish = (obj: ApiKey) => {
    obj.setTenantId(this.props.tenant.getId());

    let req = new CreateApiKeyRequest();
    req.setApiKey(obj);

    InternalStore.createApiKey(req, (resp: CreateApiKeyResponse) => {
      this.setState({
        createApiKeyResponse: resp,
      });
    });
  };

  render() {
    const apiKey = new ApiKey();

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
                  <Link to={`/tenants/${this.props.tenant.getId()}/api-keys`}>API Keys</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add API key"
        />
        <Card>
          {!this.state.createApiKeyResponse && <ApiKeyForm initialValues={apiKey} onFinish={this.onFinish} />}
          {this.state.createApiKeyResponse && <ApiKeyToken createApiKeyResponse={this.state.createApiKeyResponse} />}
        </Card>
      </Space>
    );
  }
}

export default CreateTenantApiKey;
