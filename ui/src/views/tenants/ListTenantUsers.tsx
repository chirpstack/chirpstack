import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import type { ListTenantUsersResponse, TenantUserListItem } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { ListTenantUsersRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import TenantStore from "../../stores/TenantStore";
import Admin from "../../components/Admin";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
}

function ListTenantUsers(props: IProps) {
  useTitle("Tenants", props.tenant.getName(), "Tenant users");
  const columns: ColumnsType<TenantUserListItem.AsObject> = [
    {
      title: "Email",
      dataIndex: "email",
      key: "email",
      render: (text, record) => <Link to={`/tenants/${props.tenant.getId()}/users/${record.userId}/edit`}>{text}</Link>,
    },
    {
      title: "Is tenant admin",
      dataIndex: "isAdmin",
      key: "isAdmin",
      render: (text, record) => {
        if (record.isAdmin) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Is gateway admin",
      dataIndex: "isGatewayAdmin",
      key: "isGatewayAdmin",
      render: (text, record) => {
        if (record.isGatewayAdmin) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Is device admin",
      dataIndex: "isDeviceAdmin",
      key: "isDeviceAdmin",
      render: (text, record) => {
        if (record.isDeviceAdmin) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
  ];

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    const req = new ListTenantUsersRequest();
    req.setTenantId(props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    TenantStore.listUsers(req, (resp: ListTenantUsersResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

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
              <span>Tenant users</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Tenant users"
        extra={[
          <Admin tenantId={props.tenant.getId()} isTenantAdmin>
            <Button type="primary">
              <Link to={`/tenants/${props.tenant.getId()}/users/create`}>Add tenant user</Link>
            </Button>
          </Admin>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="userId" />
    </Space>
  );
}

export default ListTenantUsers;
