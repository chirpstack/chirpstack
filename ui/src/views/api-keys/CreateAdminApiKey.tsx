import React, { useState } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { ApiKey, CreateApiKeyRequest, CreateApiKeyResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import ApiKeyForm from "./ApiKeyForm";
import ApiKeyToken from "./ApiKeyToken";
import InternalStore from "../../stores/InternalStore";

function CreateAdminApiKey() {
  const [createApiKeyResponse, setCreateApiKeyResponse] = useState<CreateApiKeyResponse | undefined>(undefined);

  const onFinish = (obj: ApiKey) => {
    obj.setIsAdmin(true);

    let req = new CreateApiKeyRequest();
    req.setApiKey(obj);

    InternalStore.createApiKey(req, (resp: CreateApiKeyResponse) => {
      setCreateApiKeyResponse(resp);
    });
  };

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
        {!createApiKeyResponse && <ApiKeyForm initialValues={apiKey} onFinish={onFinish} />}
        {createApiKeyResponse && <ApiKeyToken createApiKeyResponse={createApiKeyResponse} />}
      </Card>
    </Space>
  );
}

export default CreateAdminApiKey;
