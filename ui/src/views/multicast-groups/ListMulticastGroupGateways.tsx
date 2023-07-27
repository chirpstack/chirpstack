import React, { useState } from "react";

import { Space, Button } from "antd";
import { ColumnsType } from "antd/es/table";

import {
  ListGatewaysRequest,
  ListGatewaysResponse,
  GatewayListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import {
  MulticastGroup,
  RemoveGatewayFromMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";

interface IProps {
  application: Application;
  multicastGroup: MulticastGroup;
}

function ListMulticastGroupGateways(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);

  const columns: ColumnsType<GatewayListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "Gateway ID",
      dataIndex: "gatewayId",
      key: "gatewayId",
      width: 250,
    },
  ];

  const onRowsSelectChange = (ids: string[]) => {
    setSelectedRowIds(ids);
  };

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListGatewaysRequest();
    req.setTenantId(props.application.getTenantId());
    req.setMulticastGroupId(props.multicastGroup.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const removeGatewaysFromMulticastGroup = () => {
    if (!window.confirm("Are you sure you want to remove the selected gateways from the multicast-group?")) {
      return;
    }

    let count = 0;

    for (let gatewayId of selectedRowIds) {
      count++;

      let req = new RemoveGatewayFromMulticastGroupRequest();
      req.setMulticastGroupId(props.multicastGroup.getId());
      req.setGatewayId(gatewayId);

      let cbFunc = (cnt: number) => {
        return () => {
          if (cnt === selectedRowIds.length) {
            setRefreshKey(refreshKey + 1);
          }
        };
      };

      MulticastGroupStore.removeGateway(req, cbFunc(count));
    }
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Space direction="horizontal" style={{ float: "right" }}>
        <Button onClick={removeGatewaysFromMulticastGroup} disabled={selectedRowIds.length === 0}>
          Remove from multicast-group
        </Button>
      </Space>
      <DataTable
        columns={columns}
        getPage={getPage}
        onRowsSelectChange={onRowsSelectChange}
        rowKey="gatewayId"
        refreshKey={refreshKey}
      />
    </Space>
  );
}

export default ListMulticastGroupGateways;
