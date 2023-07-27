import React, { useState, useEffect } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import {
  Tenant,
  TenantUser,
  GetTenantUserRequest,
  GetTenantUserResponse,
  UpdateTenantUserRequest,
  DeleteTenantUserRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantUserForm from "./TenantUserForm";
import TenantStore from "../../stores/TenantStore";
import SessionStore from "../../stores/SessionStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";

function EditTenantUser({ tenant }: { tenant: Tenant }) {
  const [tenantUser, setTenantUser] = useState<TenantUser | undefined>(undefined);
  const { userId } = useParams();
  const navigate = useNavigate();

  useEffect(() => {
    let req = new GetTenantUserRequest();
    req.setTenantId(tenant.getId());
    req.setUserId(userId!);

    TenantStore.getUser(req, (resp: GetTenantUserResponse) => {
      setTenantUser(resp.getTenantUser());
    });
  }, [userId, tenant]);

  const onFinish = (obj: TenantUser) => {
    let req = new UpdateTenantUserRequest();
    req.setTenantUser(obj);

    TenantStore.updateUser(req, () => {
      navigate(`/tenants/${tenant.getId()}/users`);
    });
  };

  const deleteTenantUser = () => {
    let req = new DeleteTenantUserRequest();
    req.setTenantId(tenant.getId());
    req.setUserId(userId!);

    TenantStore.deleteUser(req, () => {
      navigate(`/tenants/${tenant.getId()}/users`);
    });
  };

  const tu = tenantUser;

  if (!tu) {
    return null;
  }

  const disabled = !(SessionStore.isAdmin() || SessionStore.isTenantAdmin(tenant.getId()));

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
              <span>{tu.getEmail()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={tu.getEmail()}
        subTitle={`user id: ${tu.getUserId()}`}
        extra={[
          <Admin tenantId={tenant.getId()} isTenantAdmin>
            <DeleteConfirm typ="tenant user" confirm={tu.getEmail()} onConfirm={deleteTenantUser}>
              <Button danger type="primary">
                Delete tenant user
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <TenantUserForm initialValues={tu} onFinish={onFinish} disabled={disabled} disableEmail />
      </Card>
    </Space>
  );
}

export default EditTenantUser;
