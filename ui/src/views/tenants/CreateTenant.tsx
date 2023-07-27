import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant, CreateTenantRequest, CreateTenantResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantForm from "./TenantForm";
import TenantStore from "../../stores/TenantStore";

function CreateTenant() {
  const navigate = useNavigate();

  const onFinish = (obj: Tenant) => {
    let req = new CreateTenantRequest();
    req.setTenant(obj);

    TenantStore.create(req, (resp: CreateTenantResponse) => {
      navigate(`/tenants/${resp.getId()}`);
    });
  };

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
        <TenantForm initialValues={tenant} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateTenant;
