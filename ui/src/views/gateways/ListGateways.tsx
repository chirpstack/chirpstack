import { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import { format } from "date-fns";
import { Space, Breadcrumb, Button, Badge, MenuProps, Modal, TreeSelect, TreeSelectProps, Dropdown } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import type { ListGatewaysResponse, GatewayListItem } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { ListGatewaysRequest, GatewayState } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import type {
  ListMulticastGroupsResponse,
  MulticastGroupListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  ListMulticastGroupsRequest,
  AddGatewayToMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import type {
  ListFuotaDeploymentsResponse,
  FuotaDeploymentListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import {
  AddGatewaysToFuotaDeploymentRequest,
  ListFuotaDeploymentsRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import FuotaStore from "../../stores/FuotaStore";
import Admin from "../../components/Admin";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
}

function ListGateways(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [multicastGroups, setMulticastGroups] = useState<TreeSelectProps["treeData"]>([]);
  const [fuotaDeployments, setFuotaDeployments] = useState<TreeSelectProps["treeData"]>([]);
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
    const mgReq = new ListMulticastGroupsRequest();
    mgReq.setLimit(999);
    mgReq.setTenantId(props.tenant.getId());
    MulticastGroupStore.list(mgReq, (resp: ListMulticastGroupsResponse) => {
      interface McGrouped {
        [key: string]: MulticastGroupListItem[];
      }
      let mgGrouped: McGrouped = {};

      for (const mg of resp.getResultList()) {
        if (mgGrouped[mg.getApplicationName()]) {
          mgGrouped[mg.getApplicationName()].push(mg);
        } else {
          mgGrouped[mg.getApplicationName()] = [mg];
        }
      }

      let mgGroups: TreeSelectProps["treeData"] = [];
      const sortedKeys = Object.keys(mgGrouped).sort();
      sortedKeys.forEach(key => {
        mgGroups.push({
          value: key,
          title: key,
          disabled: true,
          children: mgGrouped[key].map(mg => ({
            title: mg.getName(),
            value: mg.getId(),
            disabled: false,
            children: [],
          })),
        });
      });

      setMulticastGroups(mgGroups);
    });

    const fuotaReq = new ListFuotaDeploymentsRequest();
    fuotaReq.setLimit(999);
    fuotaReq.setTenantId(props.tenant.getId());
    FuotaStore.listDeployments(fuotaReq, (resp: ListFuotaDeploymentsResponse) => {
      interface FuotaGrouped {
        [key: string]: FuotaDeploymentListItem[];
      }
      let fuotaGrouped: FuotaGrouped = {};

      for (const fd of resp.getResultList()) {
        if (fuotaGrouped[fd.getApplicationName()]) {
          fuotaGrouped[fd.getApplicationName()].push(fd);
        } else {
          fuotaGrouped[fd.getApplicationName()] = [fd];
        }
      }

      let fuotaDeployments: TreeSelectProps["treeData"] = [];
      const sortedKeys = Object.keys(fuotaGrouped).sort();
      sortedKeys.forEach(key => {
        fuotaDeployments.push({
          value: key,
          title: key,
          disabled: true,
          children: fuotaGrouped[key].map(fd => ({
            title: fd.getName(),
            value: fd.getId(),
            disabled: false,
            children: [],
          })),
        });
      });

      setFuotaDeployments(fuotaDeployments);
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

  const menu: MenuProps = {
    items: [
      { key: "mg", label: "Add to multicast group", onClick: () => setMgModalVisible(true) },
      { key: "fuota", label: "Add to FUOTA deployment", onClick: () => setFuotaModalVisible(true) },
    ],
  };

  return (
    <Space orientation="vertical" style={{ width: "100%" }} size="large">
      <Modal
        title="Add selected gateways to multicast-group"
        open={mgModalVisible}
        onOk={handleMgModalOk}
        onCancel={() => setMgModalVisible(false)}
        okButtonProps={{ disabled: mgSelected === "" }}
      >
        <Space orientation="vertical" size="large" style={{ width: "100%" }}>
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
        <Space orientation="vertical" size="large" style={{ width: "100%" }}>
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
          <Breadcrumb
            items={[
              { title: "Tenants" },
              { title: <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link> },
              { title: "Gateways" },
            ]}
          />
        )}
        extra={[
          <Admin tenantId={props.tenant.getId()} isGatewayAdmin key="add-gateway">
            <Space orientation="horizontal" style={{ float: "right" }}>
              <Button type="primary">
                <Link to={`/tenants/${props.tenant.getId()}/gateways/create`}>Add gateway</Link>
              </Button>
              <Dropdown placement="bottomRight" menu={menu} trigger={["click"]} disabled={selectedRowIds.length === 0}>
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
