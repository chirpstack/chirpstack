import { Link } from "react-router-dom";

import { Space, Button } from "antd";
import type { ColumnsType } from "antd/es/table";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type {
  FuotaDeploymentListItem,
  ListFuotaDeploymentsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import { ListFuotaDeploymentsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import DataTable from "../../components/DataTable";
import type { GetPageCallbackFunc } from "../../components/DataTable";
import FuotaStore from "../../stores/FuotaStore";
import Admin from "../../components/Admin";

interface IProps {
  application: Application;
}

function ListFuotaDeployments(props: IProps) {
  const columns: ColumnsType<FuotaDeploymentListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => (
        <Link
          to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/fuota/${record.id}`}
        >
          {text}
        </Link>
      ),
    },
  ];

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListFuotaDeploymentsRequest();
    req.setApplicationId(props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    FuotaStore.listDeployments(req, (resp: ListFuotaDeploymentsResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Admin tenantId={props.application.getTenantId()} isDeviceAdmin>
        <Button type="primary" style={{ float: "right" }}>
          <Link
            to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/fuota/create`}
          >
            New FUOTA deployment
          </Link>
        </Button>
      </Admin>
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListFuotaDeployments;
