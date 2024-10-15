import { useState } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { CreateApiKeyResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { ApiKey, CreateApiKeyRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import ApiKeyForm from "./ApiKeyForm";
import ApiKeyToken from "./ApiKeyToken";
import InternalStore from "../../stores/InternalStore";
import { useTitle } from "../helpers";

function CreateAdminApiKey() {
  useTitle("Network Server", "API keys", "Add");
  const [createApiKeyResponse, setCreateApiKeyResponse] = useState<CreateApiKeyResponse | undefined>(undefined);

  const onFinish = (obj: ApiKey) => {
    obj.setIsAdmin(true);

    const req = new CreateApiKeyRequest();
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
              <span>Network Server</span>
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
