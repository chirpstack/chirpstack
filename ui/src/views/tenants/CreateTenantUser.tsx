import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { TenantUser, AddTenantUserRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantUserForm from "./TenantUserForm";
import TenantStore from "../../stores/TenantStore";
import { useTitle } from "../helpers";

function CreateTenantUser({ tenant }: { tenant: Tenant }) {
  const navigate = useNavigate();
  useTitle("Tenants", tenant.getName(), "Tenant users", "Add");

  const onFinish = (obj: TenantUser) => {
    obj.setTenantId(tenant.getId());

    const req = new AddTenantUserRequest();
    req.setTenantUser(obj);

    TenantStore.addUser(req, () => {
      navigate(`/tenants/${tenant.getId()}/users`);
    });
  };

  const tu = new TenantUser();

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb
            items={[
              { title: "Tenants" },
              { title: <Link to={`/tenants/${tenant.getId()}`}>{tenant.getName()}</Link> },
              {
                title: <Link to={`/tenants/${tenant.getId()}/users`}>Tenant users</Link>,
              },
              { title: "Add" },
            ]}
          />
        )}
        title="Add tenant user"
      />
      <Card>
        <TenantUserForm initialValues={tu} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateTenantUser;
