import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { ApiKey, CreateApiKeyRequest, CreateApiKeyResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import ApiKeyForm from "./ApiKeyForm";
import ApiKeyToken from "./ApiKeyToken";
import InternalStore from "../../stores/InternalStore";

interface IProps {}

interface IState {
  createApiKeyResponse?: CreateApiKeyResponse;
}

class CreateAdminApiKey extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  onFinish = (obj: ApiKey) => {
    obj.setIsAdmin(true);

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
          title="Add API key"
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Network-server</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to="/api-keys">API keys</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
        />
        <Card>
          {!this.state.createApiKeyResponse && <ApiKeyForm initialValues={apiKey} onFinish={this.onFinish} />}
          {this.state.createApiKeyResponse && <ApiKeyToken createApiKeyResponse={this.state.createApiKeyResponse} />}
        </Card>
      </Space>
    );
  }
}

export default CreateAdminApiKey;
