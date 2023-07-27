import { Routes, Route, Link, useNavigate, useLocation } from "react-router-dom";

import { Space, Breadcrumb, Menu, Card, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant, DeleteTenantRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";
import EditTenant from "./EditTenant";
import TenantDashboard from "./TenantDashboard";

function TenantLayout({ tenant }: { tenant: Tenant }) {
  const navigate = useNavigate();
  const location = useLocation();

  const deleteTenant = () => {
    let req = new DeleteTenantRequest();
    req.setId(tenant.getId());

    TenantStore.delete(req, () => {
      navigate("/tenants");
    });
  };

  let tab = "dashboard";
  if (location.pathname.endsWith("/edit")) {
    tab = "edit";
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Tenants</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{tenant.getName()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={tenant.getName()}
        subTitle={`tenant id: ${tenant.getId()}`}
        extra={[
          <Admin>
            <DeleteConfirm confirm={tenant.getName()} typ="tenant" onConfirm={deleteTenant}>
              <Button danger type="primary">
                Delete tenant
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />

      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="dashboard">
            <Link to={`/tenants/${tenant.getId()}`}>Dashboard</Link>
          </Menu.Item>
          <Menu.Item key="edit">
            <Link to={`/tenants/${tenant.getId()}/edit`}>Configuration</Link>
          </Menu.Item>
        </Menu>
        <Routes>
          <Route path="/" element={<TenantDashboard tenant={tenant} />} />
          <Route path="/edit" element={<EditTenant tenant={tenant} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default TenantLayout;
