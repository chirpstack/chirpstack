import { useState } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { CreateApiKeyResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { ApiKey, CreateApiKeyRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import ApiKeyForm from "./ApiKeyForm";
import ApiKeyToken from "./ApiKeyToken";
import InternalStore from "../../stores/InternalStore";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
}

function CreateTenantApiKey(props: IProps) {
  const [createApiKeyResponse, setCreateApiKeyResponse] = useState<CreateApiKeyResponse | undefined>(undefined);
  useTitle("Tenants", props.tenant.getName(), "API Keys", "Add");

  const onFinish = (obj: ApiKey) => {
    obj.setTenantId(props.tenant.getId());

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
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Building</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/api-keys`}>API Keys</Link>
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
        {!createApiKeyResponse && <ApiKeyForm initialValues={apiKey} onFinish={onFinish} />}
        {createApiKeyResponse && <ApiKeyToken createApiKeyResponse={createApiKeyResponse} />}
      </Card>
    </Space>
  );
}

export default CreateTenantApiKey;
