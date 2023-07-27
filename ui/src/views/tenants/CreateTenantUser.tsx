import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant, TenantUser, AddTenantUserRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantUserForm from "./TenantUserForm";
import TenantStore from "../../stores/TenantStore";

function CreateTenantUser({ tenant }: { tenant: Tenant }) {
  const navigate = useNavigate();

  const onFinish = (obj: TenantUser) => {
    obj.setTenantId(tenant.getId());

    let req = new AddTenantUserRequest();
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
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Tenants</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${tenant.getId()}`}>{tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${tenant.getId()}/users`}>Tenant users</Link>
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
        <TenantUserForm initialValues={tu} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateTenantUser;
