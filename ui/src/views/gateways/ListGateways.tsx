import { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import { format } from "date-fns";
import { Space, Breadcrumb, Button, Badge, Menu, Modal, TreeSelect, Dropdown } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import type { ListGatewaysResponse, GatewayListItem } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { ListGatewaysRequest, GatewayState } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import type { ListApplicationsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { ListApplicationsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type { ListMulticastGroupsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  ListMulticastGroupsRequest,
  AddGatewayToMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import type { ListFuotaDeploymentsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import {
  ListFuotaDeploymentDevicesRequest,
  AddGatewaysToFuotaDeploymentRequest,
  ListFuotaDeploymentsRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import ApplicationStore from "../../stores/ApplicationStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import FuotaStore from "../../stores/FuotaStore";
import Admin from "../../components/Admin";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
}

interface MulticastGroup {
  title: string;
  value: string;
  disabled: boolean;
  children: { title: string; value: string }[];
}

interface FuotaDeployment {
  title: string;
  value: string;
  disabled: boolean;
  children: { title: string; value: string }[];
}

function ListGateways(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [multicastGroups, setMulticastGroups] = useState<MulticastGroup[]>([]);
  const [fuotaDeployments, setFuotaDeployments] = useState<FuotaDeployment[]>([]);
  const [mgModalVisible, setMgModalVisible] = useState<boolean>(false);
  const [fuotaModalVisible, setFuotaModalVisible] = useState<boolean>(false);
  const [mgSelected, setMgSelected] = useState<string>("");
  const [fuotaDeploymentSelected, setFuotaDeploymentSelected] = useState<string>("");
  useTitle("Tenants", props.tenant.getName(), "Gateways");

  const columns: ColumnsType<GatewayListItem.AsObject> = [
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
      sorter: true,
    },
    {
      title: "Gateway ID",
      dataIndex: "gatewayId",
      key: "gatewayId",
      width: 250,
      render: (text, record) => (
        <Link to={`/tenants/${props.tenant.getId()}/gateways/${record.gatewayId}`}>{text}</Link>
      ),
      sorter: true,
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      sorter: true,
    },
    {
      title: "Region ID",
      dataIndex: "propertiesMap",
      key: "regionConfigId",
      width: 150,
      render: (text, record) => {
        for (const [k, v] of record.propertiesMap) {
          if (k === "region_config_id") {
            return <Link to={`/regions/${v}`}>{v}</Link>;
          }
        }
        return "";
      },
    },
    {
      title: "Region common-name",
      dataIndex: "propertiesMap",
      key: "regionCommonName",
      width: 250,
      render: (text, record) => {
        for (const [k, v] of record.propertiesMap) {
          if (k === "region_common_name") {
            return v;
          }
        }
        return "";
      },
    },
  ];

  useEffect(() => {
    const req = new ListApplicationsRequest();
    req.setLimit(999);
    req.setTenantId(props.tenant.getId());

    let mgGroups: MulticastGroup[] = [];
    let fDeployments: FuotaDeployment[] = [];

    ApplicationStore.list(req, (resp: ListApplicationsResponse) => {
      for (const app of resp.getResultList()) {
        const mgReq = new ListMulticastGroupsRequest();
        mgReq.setLimit(999);
        mgReq.setApplicationId(app.getId());
        MulticastGroupStore.list(mgReq, (resp: ListMulticastGroupsResponse) => {
          mgGroups.push({
            title: app.getName(),
            value: "",
            disabled: true,
            children: resp.getResultList().map((mg, i) => ({
              title: mg.getName(),
              value: mg.getId(),
            })),
          });

          // The above can also be done using setMulticastGroups and a callback
          // function, but this introduces a race-condition when executed twice.
          setMulticastGroups(mgGroups);
        });

        const fuotaReq = new ListFuotaDeploymentsRequest();
        fuotaReq.setLimit(999);
        fuotaReq.setApplicationId(app.getId());
        FuotaStore.listDeployments(fuotaReq, (resp: ListFuotaDeploymentsResponse) => {
          fDeployments.push({
            title: app.getName(),
            value: "",
            disabled: true,
            children: resp.getResultList().map((mg, i) => ({
              title: mg.getName(),
              value: mg.getId(),
            })),
          });

          // The above can also be done using setFuotaDeployments and a callback
          // function, but this introduces a race-condition when executed twice.
          setFuotaDeployments(fDeployments);
        });
      }
    });
  }, [props.tenant]);

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    function getOrderBy(orderBy: string | void): ListGatewaysRequest.OrderBy {
      switch (orderBy) {
        case "lastSeenAt":
          return ListGatewaysRequest.OrderBy.LAST_SEEN_AT;
        case "gatewayId":
          return ListGatewaysRequest.OrderBy.GATEWAY_ID;
        default:
          return ListGatewaysRequest.OrderBy.NAME;
      }
    }

    const req = new ListGatewaysRequest();
    req.setTenantId(props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);
    req.setOrderBy(getOrderBy(orderBy));
    req.setOrderByDesc(orderByDesc || false);

    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const onRowsSelectChange = (ids: string[]) => {
    setSelectedRowIds(ids);
  };

  const handleMgModalOk = () => {
    for (const gatewayId of selectedRowIds) {
      const req = new AddGatewayToMulticastGroupRequest();
      req.setMulticastGroupId(mgSelected);
      req.setGatewayId(gatewayId);

      MulticastGroupStore.addGateway(req, () => {});
    }

    setMgModalVisible(false);
  };

  const handleFuotaModalOk = () => {
    const req = new AddGatewaysToFuotaDeploymentRequest();
    req.setFuotaDeploymentId(fuotaDeploymentSelected);
    req.setGatewayIdsList(selectedRowIds);

    FuotaStore.addGateways(req, () => {
      setFuotaModalVisible(false);
    });
  };

  const menu = (
    <Menu>
      <Menu.Item key="mg" onClick={() => setMgModalVisible(true)}>
        Add to multicast-group
      </Menu.Item>
      <Menu.Item key="fuota" onClick={() => setFuotaModalVisible(true)}>
        Add to FUOTA deployment
      </Menu.Item>
    </Menu>
  );

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Modal
        title="Add selected gateways to multicast-group"
        open={mgModalVisible}
        onOk={handleMgModalOk}
        onCancel={() => setMgModalVisible(false)}
        okButtonProps={{ disabled: mgSelected === "" }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <TreeSelect
            style={{ width: "100%" }}
            placeholder="Select multicast-group"
            treeData={multicastGroups}
            onChange={v => setMgSelected(v)}
            treeDefaultExpandAll
          />
        </Space>
      </Modal>
      <Modal
        title="Add selected gateways to FUOTA deployment"
        open={fuotaModalVisible}
        onOk={handleFuotaModalOk}
        onCancel={() => setFuotaModalVisible(false)}
        okButtonProps={{ disabled: fuotaDeploymentSelected === "" }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <TreeSelect
            style={{ width: "100%" }}
            placeholder="Select FUOTA deployment"
            treeData={fuotaDeployments}
            onChange={v => setFuotaDeploymentSelected(v)}
            treeDefaultExpandAll
          />
        </Space>
      </Modal>
      <PageHeader
        title="Gateways"
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
              <span>Gateways</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        extra={[
          <Admin tenantId={props.tenant.getId()} isGatewayAdmin>
            <Space direction="horizontal" style={{ float: "right" }}>
              <Button type="primary">
                <Link to={`/tenants/${props.tenant.getId()}/gateways/create`}>Add gateway</Link>
              </Button>
              <Dropdown
                placement="bottomRight"
                overlay={menu}
                trigger={["click"]}
                disabled={selectedRowIds.length === 0}
              >
                <Button>Selected gateways</Button>
              </Dropdown>
            </Space>
          </Admin>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} onRowsSelectChange={onRowsSelectChange} rowKey="gatewayId" />
    </Space>
  );
}

export default ListGateways;
