import { Link } from "react-router-dom";

import { format } from "date-fns";
import { Space, Breadcrumb, Badge } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import type {
  ListRelayGatewaysResponse,
  RelayGatewayListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { ListRelayGatewaysRequest, GatewayState } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import type { GetPageCallbackFunc } from "../../../components/DataTable";
import DataTable from "../../../components/DataTable";
import GatewayStore from "../../../stores/GatewayStore";
import { useTitle } from "../../helpers";

interface IProps {
  tenant: Tenant;
}

function ListRelayGateways(props: IProps) {
  useTitle("Tenants", props.tenant.getName(), "Gateway Mesh", "Relay Gateways");
  const columns: ColumnsType<RelayGatewayListItem.AsObject> = [
    {
      title: "",
      dataIndex: "state",
      key: "state",
      width: 150,
      render: (text, record) => {
        if (record.state === GatewayState.NEVER_SEEN) {
          return <Badge status="warning" text="Never seen" />;
        } else if (record.state === GatewayState.OFFLINE) {
          return <Badge status="error" text="Offline" />;
        } else if (record.state === GatewayState.ONLINE) {
          return <Badge status="success" text="Online" />;
        }
      },
    },
    {
      title: "Last seen",
      dataIndex: "lastSeenAt",
      key: "lastSeenAt",
      width: 250,
      render: (text, record) => {
        if (record.lastSeenAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.lastSeenAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        }
      },
    },
    {
      title: "Relay ID",
      dataIndex: "relayId",
      key: "relayId",
      width: 250,
      render: (text, record) => (
        <Link to={`/tenants/${props.tenant.getId()}/gateways/mesh/relays/${record.relayId}/edit`}>{text}</Link>
      ),
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "Region ID",
      dataIndex: "regionConfigId",
      key: "regionConfigId",
      width: 150,
      render: text => {
        return <Link to={`/regions/${text}`}>{text}</Link>;
      },
    },
  ];

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    const req = new ListRelayGatewaysRequest();
    req.setTenantId(props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    GatewayStore.listRelayGateways(req, (resp: ListRelayGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        title="Relay Gateways"
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
              <span>Gateway Mesh</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Relay Gateways</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="relayId" />
    </Space>
  );
}

export default ListRelayGateways;
