import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import moment from "moment";
import { Space, Breadcrumb, Button, Badge, Menu, Modal, TreeSelect, Dropdown } from "antd";
import { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import {
  ListGatewaysRequest,
  ListGatewaysResponse,
  GatewayListItem,
  GatewayState,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import {
  ListApplicationsRequest,
  ListApplicationsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  ListMulticastGroupsRequest,
  ListMulticastGroupsResponse,
  AddGatewayToMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import ApplicationStore from "../../stores/ApplicationStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import Admin from "../../components/Admin";

interface IProps {
  tenant: Tenant;
}

function ListGateways(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [multicastGroups, setMulticastGroups] = useState<any[]>([]);
  const [mgModalVisible, setMgModalVisible] = useState<boolean>(false);
  const [mgSelected, setMgSelected] = useState<string>("");

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
          let ts = new Date(0);
          ts.setUTCSeconds(record.lastSeenAt.seconds);
          return moment(ts).format("YYYY-MM-DD HH:mm:ss");
        }
      },
    },
    {
      title: "Gateway ID",
      dataIndex: "gatewayId",
      key: "gatewayId",
      width: 250,
      render: (text, record) => (
        <Link to={`/tenants/${props.tenant.getId()}/gateways/${record.gatewayId}`}>{text}</Link>
      ),
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
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
    let req = new ListApplicationsRequest();
    req.setLimit(999);
    req.setTenantId(props.tenant.getId());

    ApplicationStore.list(req, (resp: ListApplicationsResponse) => {
      for (const app of resp.getResultList()) {
        let req = new ListMulticastGroupsRequest();
        req.setLimit(999);
        req.setApplicationId(app.getId());

        MulticastGroupStore.list(req, (resp: ListMulticastGroupsResponse) => {
          setMulticastGroups(m => {
            m.push({
              title: app.getName(),
              value: "",
              disabled: true,
              children: resp.getResultList().map((mg, i) => ({
                title: mg.getName(),
                value: mg.getId(),
              })),
            });
            return m;
          });
        });
      }
    });
  }, [props]);

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListGatewaysRequest();
    req.setTenantId(props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const onRowsSelectChange = (ids: string[]) => {
    setSelectedRowIds(ids);
  };

  const showMgModal = () => {
    setMgModalVisible(true);
  };

  const hideMgModal = () => {
    setMgModalVisible(false);
  };

  const onMgSelected = (value: string) => {
    setMgSelected(value);
  };

  const handleMgModalOk = () => {
    for (let gatewayId of selectedRowIds) {
      let req = new AddGatewayToMulticastGroupRequest();
      req.setMulticastGroupId(mgSelected);
      req.setGatewayId(gatewayId);

      MulticastGroupStore.addGateway(req, () => {});
    }

    setMgModalVisible(false);
  };

  const menu = (
    <Menu>
      <Menu.Item onClick={showMgModal}>Add to multicast-group</Menu.Item>
    </Menu>
  );

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Modal
        title="Add selected gateways to multicast-group"
        visible={mgModalVisible}
        onOk={handleMgModalOk}
        onCancel={hideMgModal}
        okButtonProps={{ disabled: mgSelected === "" }}
        bodyStyle={{ height: 300 }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <TreeSelect
            style={{ width: "100%" }}
            placeholder="Select multicast-group"
            treeData={multicastGroups}
            onChange={onMgSelected}
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
