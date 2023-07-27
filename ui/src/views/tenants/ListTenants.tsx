import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import {
  ListTenantsRequest,
  ListTenantsResponse,
  TenantListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import TenantStore from "../../stores/TenantStore";

function ListTenants() {
  const columns: ColumnsType<TenantListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => <Link to={`/tenants/${record.id}`}>{text}</Link>,
    },
    {
      title: "Can have gateways",
      dataIndex: "canHaveGateways",
      key: "canHaveGateways",
      width: 250,
      render: (text, record) => {
        if (record.canHaveGateways) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Private gateways (uplink)",
      dataIndex: "privateGatewaysUp",
      key: "privateGatewaysUp",
      width: 250,
      render: (text, record) => {
        if (record.privateGatewaysUp) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Private gateways (down)",
      dataIndex: "privateGatewaysDown",
      key: "privateGatewaysDown",
      width: 250,
      render: (text, record) => {
        if (record.privateGatewaysDown) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Max. gateways",
      dataIndex: "maxGatewayCount",
      key: "maxGatewayCount",
      width: 250,
      render: (text, record) => {
        if (!record.canHaveGateways) {
          return 0;
        }

        if (record.maxGatewayCount === 0) {
          return "unlimited";
        } else {
          return record.maxGatewayCount;
        }
      },
    },
    {
      title: "Max. devices",
      dataIndex: "maxDeviceCount",
      key: "maxDeviceCount",
      width: 250,
      render: (text, record) => {
        if (record.maxDeviceCount === 0) {
          return "unlimited";
        } else {
          return record.maxDeviceCount;
        }
      },
    },
  ];

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListTenantsRequest();
    req.setLimit(limit);
    req.setOffset(offset);

    TenantStore.list(req, (resp: ListTenantsResponse) => {
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
              <span>Network Server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Tenants</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Tenants"
        extra={[
          <Button type="primary">
            <Link to="/tenants/create">Add tenant</Link>
          </Button>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListTenants;
