import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import {
  ListApplicationsRequest,
  ListApplicationsResponse,
  ApplicationListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import ApplicationStore from "../../stores/ApplicationStore";
import Admin from "../../components/Admin";

interface IProps {
  tenant: Tenant;
}

function ListApplications(props: IProps) {
  const columns: ColumnsType<ApplicationListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      width: 250,
      render: (text, record) => <Link to={`/tenants/${props.tenant.getId()}/applications/${record.id}`}>{text}</Link>,
    },
    {
      title: "Description",
      dataIndex: "description",
      key: "description",
    },
  ];

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListApplicationsRequest();
    req.setTenantId(props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    ApplicationStore.list(req, (resp: ListApplicationsResponse) => {
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
              <span>Tenants</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Applications</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Applications"
        extra={[
          <Admin tenantId={props.tenant.getId()} isDeviceAdmin>
            <Button type="primary">
              <Link to={`/tenants/${props.tenant.getId()}/applications/create`}>Add application</Link>
            </Button>
          </Admin>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListApplications;
